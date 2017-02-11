use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::JSON;
use proto;

#[post("/<problem>/Boundary", data = "<boundary>")]
fn post(state: State<super::State>,
        problem: &str,
        boundary: JSON<proto::Boundary>) -> status::Custom<()> {
    // Attempt to access the problem.
    if let Some(problem) = state.lock().unwrap().get_mut(problem) {
        if problem.boundary.is_some() {
            status::Custom(Status::Conflict, ())
        } else {
            problem.boundary = Some(boundary.0);
            status::Custom(Status::Ok, ())
        }
    } else {
        status::Custom(Status::NotFound, ())
    }
}

#[put("/<problem>/Boundary", data = "<boundary>")]
fn put(state: State<super::State>,
       problem: &str,
       boundary: JSON<proto::Boundary>) -> status::Custom<()> {
    // Attempt to access the problem.
    if let Some(problem) = state.lock().unwrap().get_mut(problem) {
        // The boundary already existed, so put succeeds.
        if problem.boundary.is_some() {
            problem.boundary = Some(boundary.0);
            status::Custom(Status::Ok, ())
        // No boundary existed.
        } else {
            status::Custom(Status::Conflict, ())
        }
    // Failed to access the problem.
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

        // Post boundary to "test" before "test" exists so it should fail.
        let mut request = MockRequest::new(Method::Post, "/test/Boundary")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Boundary{
                width: 10.0,
                length: 10.0,
                point: proto::Point{ x: -5.0, y: -5.0 },
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::NotFound);

        // Add "test" to the problems.
        let mut request = MockRequest::new(Method::Post, "/test");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Put boundary to "test" which will fail because it hasn't been posted.
        let mut request = MockRequest::new(Method::Put, "/test/Boundary")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Boundary{
                width: 10.0,
                length: 10.0,
                point: proto::Point{ x: -5.0, y: -5.0 },
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Conflict);

        // Post boundary to "test".
        let mut request = MockRequest::new(Method::Post, "/test/Boundary")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Boundary{
                width: 10.0,
                length: 10.0,
                point: proto::Point{ x: -5.0, y: -5.0 },
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Post boundary to "test" again, which will fail.
        let mut request = MockRequest::new(Method::Post, "/test/Boundary")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Boundary{
                width: 10.0,
                length: 10.0,
                point: proto::Point{ x: -5.0, y: -5.0 },
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Conflict);

        // Put boundary to "test".
        let mut request = MockRequest::new(Method::Put, "/test/Boundary")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Boundary{
                width: 10.0,
                length: 10.0,
                point: proto::Point{ x: -5.0, y: -5.0 },
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);
    }
}