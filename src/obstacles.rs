use super::MAP;
use rocket::http::Status;
use rocket::response::status;

#[delete("/<name>/obstacles/<obstacle>")]
fn delete(name: &str, obstacle: &str) -> status::Custom<()> {
    // Attempt to access the problem.
    match MAP.lock().unwrap().get_mut(name) {
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