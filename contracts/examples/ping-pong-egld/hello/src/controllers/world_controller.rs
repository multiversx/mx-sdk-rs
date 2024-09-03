use actix_web::{get, HttpResponse, Responder};
use crate::views::world_view::world_view;

#[get("/world")]
async fn world() -> impl Responder {
    HttpResponse::Ok().body(world_view())
}
