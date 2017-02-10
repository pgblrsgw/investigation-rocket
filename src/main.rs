#![feature(plugin)]
#![plugin(rocket_codegen)]

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

#[derive(Default)]
struct Problem {
    obstacles: Vec<proto::Obstacle>,
}

lazy_static! {
    static ref MAP: Mutex<HashMap<String, Problem>> = Mutex::new(HashMap::new());
}

#[get("/")]
fn query_problems() -> JSON<Vec<String>> {
    JSON(MAP.lock().unwrap().keys().cloned().collect())
}

#[post("/<name>")]
fn create_problem(name: &str) -> Result<(), Status> {
    // Attempt to add the new problem.
    match MAP.lock().unwrap().entry(String::from(name)) {
        Entry::Occupied(_) => Err(Status::Conflict),
        Entry::Vacant(v) => {
            v.insert(Problem::default());
            Ok(())
        }
    }
}

fn new_mounted_rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![query_problems, create_problem])
}

fn main() {
    new_mounted_rocket().launch();
}

#[cfg(test)]
mod test {
    use super::serde_json;
    use rocket::testing::MockRequest;
    use rocket::http::{Status, Method};

    #[test]
    fn problems() {
        let rocket = super::new_mounted_rocket();

        // Get the initial empty array.
        let mut request = MockRequest::new(Method::Get, "/");
        let mut response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Write the body out as a string.
        let obstacles: Option<Vec<String>> = response.body()
            .and_then(|b| b.into_string())
            .and_then(|s| serde_json::from_str(&s).expect("Failed to parse body as JSON"));

        assert_eq!(obstacles, Some(vec![]));

        // Post a new problem called "test".
        let mut request = MockRequest::new(Method::Post, "/test");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Make sure the problem was added.
        let mut request = MockRequest::new(Method::Get, "/");
        let mut response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Write the body out as a string.
        let obstacles: Option<Vec<String>> = response.body()
            .and_then(|b| b.into_string())
            .and_then(|s| serde_json::from_str(&s).expect("Failed to parse body as JSON"));

        assert_eq!(obstacles, Some(vec![String::from("test")]));
    }
}
