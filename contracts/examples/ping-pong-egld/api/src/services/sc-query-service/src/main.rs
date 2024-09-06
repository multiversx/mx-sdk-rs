#![allow(non_snake_case)]

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod controller;
mod model;
mod view;

async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Greetings!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(controller::query_controller::timestamp)
            .service(controller::query_controller::deadline) 
            .service(controller::query_controller::ping_amount)
            .service(controller::query_controller::max_funds)
            .service(controller::query_controller::user_addresses) 
            .route("/", web::get().to(default_route))
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}
