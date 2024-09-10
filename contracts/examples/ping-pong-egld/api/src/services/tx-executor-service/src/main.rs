#![allow(non_snake_case)]

pub mod controller;
pub mod model;
pub mod view;

use actix_cors::Cors;
use actix_web::*;
use dotenv::dotenv;
use redis::{Client, Commands};
use std::env;

async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mut redis_client = Client::open(env::var("REDIS_URL").unwrap()).expect("Invalid URL");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(redis_client.clone()))
            .route("/ping", web::post().to(controller::tx_controller::ping))
            .route("/deploy", web::post().to(controller::tx_controller::deploy))
            .route("/pong", web::post().to(controller::tx_controller::pong))
            .route("/", web::get().to(default_route))
    })
    .bind(("localhost", 8002))?
    .run()
    .await
}
