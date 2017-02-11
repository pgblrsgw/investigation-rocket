use super::Problem;
use std::collections::hash_map::Entry;
use rocket_contrib::JSON;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;

#[get("/")]
fn get(state: State<super::State>) -> JSON<Vec<String>> {
    JSON(state.lock().unwrap().keys().cloned().collect())
}

#[post("/<problem>")]
fn post(state: State<super::State>, problem: &str) -> status::Custom<()> {
    // Attempt to add the new problem.
    match state.lock().unwrap().entry(String::from(problem)) {
        Entry::Occupied(_) => status::Custom(Status::Conflict, ()),
        Entry::Vacant(v) => {
            v.insert(Problem::default());
            status::Custom(Status::Ok, ())
        }
    }
}

#[delete("/<problem>")]
fn delete(state: State<super::State>, problem: &str) -> status::Custom<()> {
    // Attempt to remove the problem.
    match state.lock().unwrap().remove(problem) {
        Some(_) => status::Custom(Status::Ok, ()),
        None => status::Custom(Status::NotFound, ()),
    }
}

#[cfg(test)]
mod test {
    use serde;
    use serde_json;
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
    fn test() {
        // Make the mock server.
        let rocket = super::super::new_mounted_rocket();

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
