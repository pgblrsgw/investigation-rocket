#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
extern crate a4_proto as proto;
#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate rocket_contrib;

mod obstacles;
mod problems;

use std::collections::hash_map::HashMap;
use std::sync::Mutex;

#[derive(Default)]
struct Problem {
    obstacles: HashMap<String, proto::Obstacle>,
}

lazy_static! {
    static ref MAP: Mutex<HashMap<String, Problem>> = Mutex::new(HashMap::new());
}

fn new_mounted_rocket() -> rocket::Rocket {
    rocket::ignite().mount("/",
                           routes![
        problems::get,
        problems::post,
        problems::delete,
        obstacles::delete,
        ])
}

fn main() {
    new_mounted_rocket().launch();
}
