use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::JSON;
use proto;

#[get("/<problem>/Path")]
fn get(state: State<super::State>, problem: &str)
        -> Result<JSON<proto::Path>, status::Custom<()>> {
    // Attempt to access the problem.
    if let Some(problem) = state.lock().unwrap().get_mut(problem) {
        // TODO: Implement.
        unimplemented!()
    } else {
        Err(status::Custom(Status::NotFound, ()))
    }
}