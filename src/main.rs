#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
extern crate a4_proto as proto;
extern crate rocket;
extern crate rocket_contrib;

mod obstacles;
mod problems;

use std::collections::hash_map::HashMap;
use std::sync::Mutex;

type State = Mutex<HashMap<String, Problem>>;

#[derive(Default)]
struct Problem {
    obstacles: HashMap<String, proto::Obstacle>,
}

fn new_mounted_rocket() -> rocket::Rocket {
    rocket::ignite().mount("/",
                           routes![
        problems::get,
        problems::post,
        problems::delete,
        obstacles::delete,
        ])
        .manage(Mutex::new(HashMap::<String, Problem>::new()))
}

fn main() {
    new_mounted_rocket().launch();
}
