use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::JSON;
use proto;
use super::Problem;

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
    use ndarray::Array2;
    use std::convert::TryFrom;
    use pathfinding;
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

    let from_coords = move |coord: [usize; 2]| -> [f64; 2] {
        [coord[0] as f64 * grid_granularity + boundary.point.x,
            coord[1] as f64 * grid_granularity + boundary.point.y]
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

    let checked_coord_add = |coord: [usize; 2], add: [isize; 2]| -> Option<[usize; 2]> {
        let new_coord = [coord[0] as isize + add[0], coord[1] as isize + add[1]];
        if let (Ok(x), Ok(y)) = (usize::try_from(new_coord[0]), usize::try_from(new_coord[1])) {
            if x < grid_width && y < grid_height {
                Some([x, y])
            } else {
                None
            }
        } else {
            None
        }
    };

    pathfinding::bfs(
        &to_coords([robot.point.x, robot.point.y]),
        |coord| {
            vec![[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]]
                .into_iter()
                .map(|c| checked_coord_add(*coord, c))
                .flatten()
                .filter_map(|c| if grid[c] {None} else {Some(c)})
                .collect_vec()
                .into_iter()
        },
        |coord| *coord == to_coords([goal.point.x, goal.point.y])
    ).map(|v| proto::Path{ points: v.into_iter().map(from_coords)
        .map(|c| proto::Point{ x: c[0], y: c[1] }).collect()})
        .ok_or(())
}

#[cfg(test)]
mod test {
    extern crate serde;
    extern crate serde_json;
    use rocket::testing::MockRequest;
    use rocket::http::{Status, Method, ContentType};
    use rocket::Response;
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

        // Add "test" to the problems.
        let mut request = MockRequest::new(Method::Post, "/test");
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

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

        // Post robot to "test".
        let mut request = MockRequest::new(Method::Post, "/test/Robot")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Robot{
                point: proto::Point{ x: -1.0, y: -1.0 },
                radius: 0.2,
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Post goal to "test".
        let mut request = MockRequest::new(Method::Post, "/test/Goal")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&proto::Goal{
                point: proto::Point{ x: 2.0, y: 2.0 },
            }).unwrap());
        let response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        // Access the path.
        let mut request = MockRequest::new(Method::Get, "/test/Path");
        let mut response = request.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);
        assert!(body_deser::<proto::Path>(&mut response).is_some());
    }
}
