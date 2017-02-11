use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::JSON;
use std::collections::hash_map::Entry;
use proto;

#[get("/<problem>/Obstacles")]
fn get_all(state: State<super::State>, problem: &str) -> Result<JSON<Vec<String>>, status::Custom<()>> {
    // Attempt to access the problem.
    match state.lock().unwrap().get_mut(problem) {
        Some(problem) => Ok(JSON(problem.obstacles.keys().cloned().collect())),
        None => Err(status::Custom(Status::NotFound, ())),
    }
}

#[delete("/<problem>/Obstacles/<obstacle_id>")]
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

#[post("/<problem>/Obstacles/<obstacle_id>", data = "<obstacle>")]
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

#[put("/<problem>/Obstacles/<obstacle_id>", data = "<obstacle>")]
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
    use rocket::Response;
    use rocket::http::{Status, Method, ContentType};
    use proto;

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
        let mut request = MockRequest::new(Method::Delete, "/test/Obstacles/asd");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::NotFound);

        // Add "test" to the problems.
        let mut request = MockRequest::new(Method::Post, "/test");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Attempt to remove a non-existing obstacle from "test".
        let mut request = MockRequest::new(Method::Delete, "/test/Obstacles/asd");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::NotFound);

        // Put obstacle "asd" to "test", which will fail because it doesn't exist yet.
        let mut request = MockRequest::new(Method::Put, "/test/Obstacles/asd")
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
        let mut request = MockRequest::new(Method::Post, "/test/Obstacles/asd")
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
        let mut request = MockRequest::new(Method::Put, "/test/Obstacles/asd")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Obstacle{
                width: 1.0,
                length: 1.0,
                coordinate_x: 0.0,
                coordinate_y: 0.0,
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Get a list of the obstacles in "test".
        // Make sure "test" was added to the array.
        let mut request = MockRequest::new(Method::Get, "/test/Obstacles");
        let mut response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(body_deser(&mut response), Some(vec![String::from("asd")]));
    }
}
