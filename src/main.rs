#![feature(plugin, try_from)]
#![plugin(rocket_codegen)]

extern crate a4_proto as proto;
extern crate rocket;
extern crate rocket_contrib;
extern crate pathfinding;
extern crate ndarray;
extern crate itertools;

mod obstacles;
mod problems;
mod robot;
mod goal;
mod path;
mod boundary;

use std::collections::hash_map::HashMap;
use std::sync::Mutex;

type State = Mutex<HashMap<String, Problem>>;

#[derive(Default)]
struct Problem {
    obstacles: HashMap<String, proto::Obstacle>,
    robot: Option<proto::Robot>,
    goal: Option<proto::Goal>,
    boundary: Option<proto::Boundary>,
}

fn new_mounted_rocket() -> rocket::Rocket {
    rocket::ignite().mount("/",
                           routes![
        problems::get,
        problems::post,
        problems::delete,
        obstacles::get_all,
        obstacles::delete,
        obstacles::post,
        obstacles::put,
        robot::post,
        robot::put,
        goal::post,
        goal::put,
        boundary::post,
        boundary::put,
        path::get,
        ])
        .manage(Mutex::new(HashMap::<String, Problem>::new()))
}

fn main() {
    new_mounted_rocket().launch();
}
