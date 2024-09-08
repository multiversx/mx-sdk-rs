use crate::model::{
    deadline_model::fetch_deadline, max_funds_model::fetch_max_funds,
    ping_amount_model::fetch_ping_amount, timestamp_model::fetch_timestamp,
    user_addresses_model::fetch_user_addresses,
};
use crate::view::{
    deadline_view::DeadlineResponse, max_funds_view::MaxFundsResponse,
    ping_amount_view::PingAmountResponse, timestamp_view::TimestampResponse,
    user_addresses_view::UserAddressesResponse,
};
use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/timestamp")]
async fn timestamp() -> impl Responder {
    // Model raw response
    match fetch_timestamp().await {
        Ok(response) => {
            let view = TimestampResponse::new(response);
            HttpResponse::Ok().json(view.response())
        },
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({ "error": "Failed to fetch timestamp" })),
    }
}

#[get("/deadline")]
async fn deadline() -> impl Responder {
    // Model raw response
    match fetch_deadline().await {
        Ok(response) => {
            let view = DeadlineResponse::new(response);
            HttpResponse::Ok().json(view.response())
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to fetch deadline" }))
        },
    }
}

#[get("/ping_amount")]
async fn ping_amount() -> impl Responder {
    // Model raw response
    match fetch_ping_amount().await {
        Ok(response) => {
            let view = PingAmountResponse::new(response);
            HttpResponse::Ok().json(view.response())
        },
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({ "error": "Failed to fetch pingAmount" })),
    }
}

#[get("/max_funds")]
async fn max_funds() -> impl Responder {
    // Model raw response
    match fetch_max_funds().await {
        Ok(response) => {
            let view = MaxFundsResponse::new(response);
            HttpResponse::Ok().json(view.response())
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to fetch maxFunds" }))
        },
    }
}

#[get("/user_addresses")]
async fn user_addresses() -> impl Responder {
    // Model raw response
    match fetch_user_addresses().await {
        Ok(response) => {
            let view = UserAddressesResponse::new(response);
            HttpResponse::Ok().json(view.response())
        },
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({ "error": "Failed to fetch user addresses" })),
    }
}
