use rocket::http::Status;
use rocket::response::status;
use rocket::State;

#[delete("/<name>/obstacles/<obstacle>")]
fn delete(state: State<super::State>, name: &str, obstacle: &str) -> status::Custom<()> {
    // Attempt to access the problem.
    match state.lock().unwrap().get_mut(name) {
        Some(problem) => {
            // Attempt to remove the obstacle.
            match problem.obstacles.remove(obstacle) {
                Some(_) => status::Custom(Status::Ok, ()),
                None => status::Custom(Status::NotFound, ()),
            }
        }
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
    }
}
