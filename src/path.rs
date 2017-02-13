use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::JSON;
use proto;
use super::Problem;

#[get("/<problem>/Path")]
fn get(state: State<super::State>, problem: &str)
        -> Result<JSON<proto::Path>, status::Custom<()>> {
    // Attempt to access the problem.
    if let Some(problem) = state.lock().unwrap().get(problem) {
        get_path(problem).map(JSON).map_err(|_| status::Custom(Status::BadRequest, ()))
    } else {
        Err(status::Custom(Status::NotFound, ()))
    }
}

fn get_path(problem: &Problem) -> Result<proto::Path, ()> {
    let robot = problem.robot.as_ref().ok_or(())?;
    let goal = problem.goal.as_ref().ok_or(())?;
    let boundary = problem.boundary.as_ref().ok_or(())?;
    unreachable!()
}