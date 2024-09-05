#![allow(non_snake_case)]

mod controllers;
use actix_cors::Cors;
use actix_web::*;

async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080") // Specify the frontend's origin
                    .allowed_methods(vec!["GET", "POST"]) // Allowed HTTP methods
                    .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600), // Cache the CORS headers for an hour
            )
            .route(
                "/tx/{tx_type}",
                web::post().to(controllers::api_controller::tx),
            )
            .route(
                "/query/{query_type}",
                web::get().to(controllers::api_controller::query),
            )
            .route("/", web::get().to(default_route))
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}
