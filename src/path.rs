use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::JSON;
use proto;
use super::Problem;
use ndarray::Array2;

use itertools::Itertools;

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

    let grid_granularity = robot.radius / 4.0;
    let grid_height = (boundary.length / grid_granularity) as usize;
    let grid_width = (boundary.width / grid_granularity) as usize;
    let mut grid = Array2::from_elem((grid_width, grid_height), false);

    let to_coords = move |pos: [f64; 2]| -> [usize; 2] {
        [((pos[0] - boundary.point.x) / grid_granularity + 0.5) as usize,
            ((pos[1] - boundary.point.y) / grid_granularity + 0.5) as usize]
    };

    for obstacle in problem.obstacles.values() {
        let start = to_coords(
            [obstacle.point.x - robot.radius, obstacle.point.y - robot.radius]
        );
        let end = to_coords(
            [obstacle.point.x + obstacle.width + robot.radius,
                obstacle.point.y + obstacle.length + robot.radius]
        );
        for (x, y) in (start[0]..end[0] + 1).cartesian_product((start[1]..end[1] + 1)) {
            grid[[x, y]] = true;
        }
    }
    unimplemented!()
}