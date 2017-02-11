#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate a4_proto as proto;
extern crate rocket;
extern crate rocket_contrib;

mod obstacles;
mod problems;
mod robot;

use std::collections::hash_map::HashMap;
use std::sync::Mutex;

type State = Mutex<HashMap<String, Problem>>;

#[derive(Default)]
struct Problem {
    obstacles: HashMap<String, proto::Obstacle>,
    robot: Option<proto::Robot>,
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
        ])
        .manage(Mutex::new(HashMap::<String, Problem>::new()))
}

fn main() {
    new_mounted_rocket().launch();
}
