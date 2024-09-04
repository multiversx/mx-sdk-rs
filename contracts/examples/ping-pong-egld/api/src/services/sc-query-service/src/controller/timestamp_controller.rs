use actix_web::{get, HttpResponse, Responder};
use serde_json::json;
use crate::model::timestamp_model::fetch_timestamp;
use crate::view::timestamp_view::TimestampView;

#[get("/timestamp")]
async fn timestamp() -> impl Responder {
    // Model raw response
    match fetch_timestamp().await {
        Ok(response) => {
            let view = TimestampView::from_response(response);
            HttpResponse::Ok().json(view.data())
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to fetch timestamp" }))
        }
    }
}
