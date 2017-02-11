use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::JSON;
use proto;

#[post("/<problem>/Goal", data = "<goal>")]
fn post(state: State<super::State>,
        problem: &str,
        goal: JSON<proto::Goal>) -> status::Custom<()> {
    // Attempt to access the problem.
    if let Some(problem) = state.lock().unwrap().get_mut(problem) {
        if problem.goal.is_some() {
            status::Custom(Status::Conflict, ())
        } else {
            problem.goal = Some(goal.0);
            status::Custom(Status::Ok, ())
        }
    } else {
        status::Custom(Status::NotFound, ())
    }
}

#[put("/<problem>/Goal", data = "<goal>")]
fn put(state: State<super::State>,
       problem: &str,
       goal: JSON<proto::Goal>) -> status::Custom<()> {
    // Attempt to access the problem.
    if let Some(problem) = state.lock().unwrap().get_mut(problem) {
        if problem.goal.is_some() {
            problem.goal = Some(goal.0);
            status::Custom(Status::Ok, ())
        } else {
            status::Custom(Status::Conflict, ())
        }
    } else {
        status::Custom(Status::NotFound, ())
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

        // Post goal to "test" before "test" exists so it should fail.
        let mut request = MockRequest::new(Method::Post, "/test/Goal")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Goal{
                point: proto::Point{ x: 0.0, y: 5.0 },
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::NotFound);

        // Add "test" to the problems.
        let mut request = MockRequest::new(Method::Post, "/test");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Put goal to "test" which will fail because it hasn't been posted.
        let mut request = MockRequest::new(Method::Put, "/test/Goal")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Goal{
                point: proto::Point{ x: 0.0, y: 5.0 },
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Conflict);

        // Post goal to "test".
        let mut request = MockRequest::new(Method::Post, "/test/Goal")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Goal{
                point: proto::Point{ x: 0.0, y: 5.0 },
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Post goal to "test" again, which will fail.
        let mut request = MockRequest::new(Method::Post, "/test/Goal")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Goal{
                point: proto::Point{ x: 0.0, y: 5.0 },
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Conflict);

        // Put goal to "test".
        let mut request = MockRequest::new(Method::Put, "/test/Goal")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Goal{
                point: proto::Point{ x: 0.0, y: 5.0 },
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);
    }
}