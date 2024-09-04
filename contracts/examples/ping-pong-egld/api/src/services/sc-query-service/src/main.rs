#![allow(non_snake_case)]

// mod controllers;
// mod models;
// mod views;

use rocket::*;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[get("/")]
fn default_route() -> &'static str {
    "Placeholder instead of gateway error :)"
}

#[launch]
fn rocket() -> _ {
    // CORS config
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8000"]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![rocket::http::Method::Get, rocket::http::Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: rocket_cors::AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS fairing");

    rocket::build()
        .configure(config::Config::figment().merge(("port", 8001)))
        .mount("/", routes![default_route]) // http://127.0.0.1:8001
        .attach(cors)
}
