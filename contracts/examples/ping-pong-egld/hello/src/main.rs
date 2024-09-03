mod controllers;
mod models;
mod views;

use actix_web::{web, App, HttpServer};
use models::basic_interact;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut interactor = basic_interact::ActixInteractor::init().await;

    // interactor.deploy().await;

    HttpServer::new(move || {
        App::new()
            .service(controllers::timestamp_controller::timestamp)
            .service(controllers::world_controller::world)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
