use actix_web::{get, HttpResponse, Responder};
use serde_json::json;
use crate::model::{
    timestamp_model::fetch_timestamp,
    deadline_model::fetch_deadline,
    ping_amount_model::fetch_ping_amount,
    max_funds_model::fetch_max_funds,
    user_addresses_model::fetch_user_addresses,
};
use crate::view::{
    timestamp_view::TimestampResponse,
    deadline_view::DeadlineResponse,   
    ping_amount_view::PingAmountResponse,
    max_funds_view::MaxFundsResponse,
    user_addresses_view::UserAddressesResponse,
};

#[get("/timestamp")]
async fn timestamp() -> impl Responder {
    // Model raw response
    match fetch_timestamp().await {
        Ok(response) => {
            let view = TimestampResponse::new(response);
            HttpResponse::Ok().json(view.response())
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to fetch timestamp" }))
        }
    }
}

#[get("/deadline")]
async fn deadline() -> impl Responder {
    // Model raw response
    match fetch_deadline().await {
        Ok(response) => {
            let view = DeadlineResponse::new(response);
            HttpResponse::Ok().json(view.response())
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to fetch deadline" }))
        }
    }
}

#[get("/ping_amount")]
async fn ping_amount() -> impl Responder {
    // Model raw response
    match fetch_ping_amount().await {
        Ok(response) => {
            let view = PingAmountResponse::new(response);
            HttpResponse::Ok().json(view.response())
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to fetch pingAmount" }))
        }
    }
}

#[get("/max_funds")]
async fn max_funds() -> impl Responder {
    // Model raw response
    match fetch_max_funds().await {
        Ok(response) => {
            let view = MaxFundsResponse::new(response);
            HttpResponse::Ok().json(view.response())
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to fetch maxFunds" }))
        }
    }
}
#[get("/user_addresses")]
async fn user_addresses() -> impl Responder {
    // Model raw response
    match fetch_user_addresses().await {
        Ok(response) => {
            let view = UserAddressesResponse::new(response);
            
            if view.is_empty() {
                HttpResponse::Ok().json(json!({ "message": "userAddresses is empty" }))
            } else {
                HttpResponse::Ok().json(view.response())
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to fetch user addresses" }))
        }
    }
}
