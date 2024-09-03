#![allow(non_snake_case)]

mod controllers;
mod models;
mod views;

use rocket::*;

#[get("/")]
fn default_route() -> &'static str {
    "Placeholder instead of gateway error :)"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![default_route]) // http://127.0.0.1:8000
        .mount("/", routes![controllers::world_controller::world]) // http://127.0.0.1:8000/world
        .mount("/", routes![controllers::timestamp_controller::timestamp]) // http://127.0.0.1:8000/timestamp
}
