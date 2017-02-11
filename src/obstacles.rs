use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::JSON;
use std::collections::hash_map::Entry;
use proto;

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

#[put("/<problem>/obstacles/<obstacle_id>", data = "<obstacle>")]
fn put(state: State<super::State>,
        problem: &str,
        obstacle_id: &str,
        obstacle: JSON<proto::Obstacle>) -> status::Custom<()> {
    // Attempt to access the problem.
    match state.lock().unwrap().get_mut(problem) {
        Some(problem) => {
            // Attempt to remove the obstacle.
            match problem.obstacles.entry(String::from(obstacle_id)) {
                Entry::Occupied(mut o) => {
                    o.insert(obstacle.0);
                    status::Custom(Status::Ok, ())
                },
                Entry::Vacant(_) => status::Custom(Status::NotFound, ()),
            }
        }
        None => status::Custom(Status::NotFound, ()),
    }
}

#[cfg(test)]
mod test {
    extern crate serde;
    extern crate serde_json;
    use rocket::testing::MockRequest;
    use rocket::http::{Status, Method, ContentType};
    use proto;

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

        // Put obstacle "asd" to "test", which will fail because it doesn't exist yet.
        let mut request = MockRequest::new(Method::Put, "/test/obstacles/asd")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Obstacle{
                width: 1.0,
                length: 1.0,
                coordinate_x: 0.0,
                coordinate_y: 0.0,
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::NotFound);

        // Add obstacle "asd" to "test".
        let mut request = MockRequest::new(Method::Post, "/test/obstacles/asd")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Obstacle{
                width: 1.0,
                length: 1.0,
                coordinate_x: 0.0,
                coordinate_y: 0.0,
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Put obstacle "asd" to "test", which will pass because it was just created.
        let mut request = MockRequest::new(Method::Put, "/test/obstacles/asd")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Obstacle{
                width: 1.0,
                length: 1.0,
                coordinate_x: 0.0,
                coordinate_y: 0.0,
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);
    }
}
