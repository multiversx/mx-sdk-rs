#![allow(non_snake_case)]

use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use redis::Client;
mod controller;
mod model;
mod view;

async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Greetings!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let redis_client = Client::open(env::var("REDIS_URL").unwrap()).expect("Invalid URL");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_client.clone()))
            .service(controller::query_controller::timestamp)
            .service(controller::query_controller::deadline)
            .service(controller::query_controller::ping_amount)
            .service(controller::query_controller::max_funds)
            .service(controller::query_controller::user_addresses)
            .service(controller::query_controller::contract_address)
            .route("/", web::get().to(default_route))
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}
