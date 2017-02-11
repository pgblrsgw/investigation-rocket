use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::JSON;
use std::collections::hash_map::Entry;
use proto;

#[post("/<problem>/obstacles/<obstacle_id>", data = "<obstacle>")]
fn post(state: State<super::State>,
        problem: &str,
        obstacle_id: &str,
        obstacle: JSON<proto::Obstacle>) -> status::Custom<()> {
    // Attempt to access the problem.
    match state.lock().unwrap().get_mut(problem) {
        Some(problem) => {
            // Attempt to remove the obstacle.
            match problem.obstacles.entry(String::from(obstacle_id)) {
                Entry::Occupied(_) => status::Custom(Status::Conflict, ()),
                Entry::Vacant(v) => {
                    v.insert(obstacle.0);
                    status::Custom(Status::Ok, ())
                }
            }
        }
        None => status::Custom(Status::NotFound, ()),
    }
}

#[delete("/<problem>/obstacles/<obstacle_id>")]
fn delete(state: State<super::State>, problem: &str, obstacle_id: &str) -> status::Custom<()> {
    // Attempt to access the problem.
    match state.lock().unwrap().get_mut(problem) {
        Some(problem) => {
            // Attempt to remove the obstacle.
            match problem.obstacles.remove(obstacle_id) {
                Some(_) => status::Custom(Status::Ok, ()),
                None => status::Custom(Status::NotFound, ()),
            }
        }
        None => status::Custom(Status::NotFound, ()),
    }
}

#[cfg(test)]
mod test {
    extern crate serde;
    extern crate serde_json;
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

        // Attempt to remove a non-existing obstacle from a non-existing problem.
        let mut request = MockRequest::new(Method::Delete, "/test/obstacles/asd");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::NotFound);

        // Add "test" to the problems.
        let mut request = MockRequest::new(Method::Post, "/test");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Attempt to remove a non-existing obstacle from "test".
        let mut request = MockRequest::new(Method::Delete, "/test/obstacles/asd");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::NotFound);
    }
}
