#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
extern crate a4_proto as proto;
#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate rocket_contrib;

use std::collections::hash_map::{HashMap, Entry};
use std::sync::Mutex;
use rocket_contrib::JSON;
use rocket::http::Status;
use rocket::response::status;

#[derive(Default)]
struct Problem {
    obstacles: HashMap<String, proto::Obstacle>,
}

lazy_static! {
    static ref MAP: Mutex<HashMap<String, Problem>> = Mutex::new(HashMap::new());
}

#[get("/")]
fn query_problems() -> JSON<Vec<String>> {
    JSON(MAP.lock().unwrap().keys().cloned().collect())
}

#[post("/<name>")]
fn create_problem(name: &str) -> status::Custom<()> {
    // Attempt to add the new problem.
    match MAP.lock().unwrap().entry(String::from(name)) {
        Entry::Occupied(_) => status::Custom(Status::Conflict, ()),
        Entry::Vacant(v) => {
            v.insert(Problem::default());
            status::Custom(Status::Ok, ())
        }
    }
}

#[delete("/<name>")]
fn delete_problem(name: &str) -> status::Custom<()> {
    // Attempt to remove the problem.
    match MAP.lock().unwrap().remove(name) {
        Some(_) => status::Custom(Status::Ok, ()),
        None => status::Custom(Status::NotFound, ()),
    }
}

fn new_mounted_rocket() -> rocket::Rocket {
    rocket::ignite().mount("/",
                           routes![
        query_problems,
        create_problem,
        delete_problem,
        ])
}

fn main() {
    new_mounted_rocket().launch();
}

#[cfg(test)]
mod test {
    use super::serde;
    use super::serde_json;
    use rocket::Response;
    use rocket::testing::MockRequest;
    use rocket::http::{Status, Method};

    fn body_deser<T: serde::Deserialize>(response: &mut Response) -> Option<T> {
        response.body()
            .and_then(|b| b.into_string())
            .map(|s| {
                serde_json::from_str(&s)
                    .unwrap_or_else(|e| panic!("Failed to parse body as JSON: {:?}", e))
            })
    }

    #[test]
    fn problems() {
        // Make the mock server.
        let rocket = super::new_mounted_rocket();

        // Get the initial empty array.
        let mut request = MockRequest::new(Method::Get, "/");
        let mut response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(body_deser::<Vec<String>>(&mut response), Some(vec![]));

        // Post a new problem called "test".
        let mut request = MockRequest::new(Method::Post, "/test");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Attempt to create test again
        let mut request = MockRequest::new(Method::Post, "/test");
        let response = request.dispatch_with(&rocket);
        // It should fail with a conflict this time.
        assert_eq!(response.status(), Status::Conflict);

        // Make sure "test" was added to the array.
        let mut request = MockRequest::new(Method::Get, "/");
        let mut response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(body_deser(&mut response), Some(vec![String::from("test")]));

        // Remove a non-existing problem "blah".
        let mut request = MockRequest::new(Method::Delete, "/blah");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::NotFound);

        // Remove "test".
        let mut request = MockRequest::new(Method::Delete, "/test");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Make sure "test" was removed from the array.
        let mut request = MockRequest::new(Method::Get, "/");
        let mut response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(body_deser::<Vec<String>>(&mut response), Some(vec![]));
    }
}
