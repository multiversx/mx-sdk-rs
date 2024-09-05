#![allow(non_snake_case)]

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod controller;
mod model;
mod view;

async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Ee Aa Aa")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(controller::timestamp_controller::timestamp)
            .route("/", web::get().to(default_route))
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}
