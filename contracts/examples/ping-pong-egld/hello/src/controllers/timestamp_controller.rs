use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::basic_interact::ActixInteractor;
use crate::views::timestamp_view::timestamp_view;

#[get("/timestamp")]
async fn timestamp() -> impl Responder {
    // Initialize ActixInteractor and call activation_timestamp()
    let mut interactor = ActixInteractor::init().await;

    interactor.deploy().await;

    println!("JAMBO");
    // Call activation_timestamp and get the result
    let response = interactor.activation_timestamp().await;

    println!("MAmbo");
    // Use the response to generate the HTTP response
    HttpResponse::Ok().body(timestamp_view(response))
}