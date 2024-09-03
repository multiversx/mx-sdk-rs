#![allow(non_snake_case)]

mod basic_interact;

use rocket::*;

#[cfg(test)] mod tests;

// Try visiting:
//   http://127.0.0.1:8000/hello/timestamp
#[get("/timestamp")]
async fn timestamp() -> &'static str {
    let mut basic_interact: basic_interact::RocketInteractor = basic_interact::RocketInteractor::init().await;

    // Use the activation_timestamp() method and handle potential errors
    //let aux = basic_interact.activation_timestamp().await;
    //println!("{:?}", aux);
    "Why"
}

// Try visiting:
//   http://127.0.0.1:8000/world
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![world])
        .mount("/", routes![timestamp])
}
