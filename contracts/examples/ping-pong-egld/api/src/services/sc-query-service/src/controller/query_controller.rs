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
use actix_web::{get, web, HttpResponse, Responder};
use redis::{Client, Commands, RedisError};
use serde_json::json;

#[get("/timestamp")]
async fn timestamp(redis_client: web::Data<Client>) -> impl Responder {
    let mut con = redis_client.get_connection().unwrap();

    let timestamp_redis_value: Result<String, RedisError> = con.get("timestamp");

    match timestamp_redis_value {
        Ok(timestamp) => HttpResponse::Ok().json(TimestampResponse::new(timestamp).response()),
        Err(_) => match fetch_timestamp().await {
            Ok(response) => {
                println!("Setting timestamp in redis");
                let _: () = con.set("timestamp", &response).unwrap();
                let view = TimestampResponse::new(response);
                HttpResponse::Ok().json(view.response())
            },
            Err(_) => HttpResponse::InternalServerError()
                .json(json!({ "error": "Failed to fetch timestamp" })),
        },
    }
}

#[get("/deadline")]
async fn deadline(redis_client: web::Data<Client>) -> impl Responder {
    let mut con = redis_client.get_connection().unwrap();

    let deadline_redis_value: Result<String, RedisError> = con.get("deadline");

    match deadline_redis_value {
        Ok(deadline) => HttpResponse::Ok().json(DeadlineResponse::new(deadline).response()),
        Err(_) => match fetch_deadline().await {
            Ok(response) => {
                println!("Setting deadline in redis");
                let _: () = con.set("deadline", &response).unwrap();
                let view = DeadlineResponse::new(response);
                HttpResponse::Ok().json(view.response())
            },
            Err(_) => HttpResponse::InternalServerError()
                .json(json!({ "error": "Failed to fetch deadline" })),
        },
    }
}

#[get("/ping_amount")]
async fn ping_amount(redis_client: web::Data<Client>) -> impl Responder {
    let mut con = redis_client.get_connection().unwrap();

    let ping_amount_redis_value: Result<String, RedisError> = con.get("ping_amount");
    match ping_amount_redis_value {
        Ok(ping_amount) => HttpResponse::Ok().json(PingAmountResponse::new(ping_amount).response()),
        Err(_) => match fetch_ping_amount().await {
            Ok(response) => {
                println!("Setting ping amount in redis");
                let _: () = con.set("ping_amount", &response).unwrap();
                let view = PingAmountResponse::new(response);
                HttpResponse::Ok().json(view.response())
            },
            Err(_) => HttpResponse::InternalServerError()
                .json(json!({ "error": "Failed to fetch pingAmount" })),
        },
    }
}

#[get("/max_funds")]
async fn max_funds(redis_client: web::Data<Client>) -> impl Responder {
    let mut con = redis_client.get_connection().unwrap();

    let max_funds_redis_value: Result<String, RedisError> = con.get("max_funds");

    match max_funds_redis_value {
        Ok(max_funds) => HttpResponse::Ok().json(MaxFundsResponse::new(max_funds).response()),
        Err(_) => match fetch_max_funds().await {
            Ok(response) => {
                println!("Setting max funds in redis");
                let _: () = con.set("max_funds", &response).unwrap();
                let view = MaxFundsResponse::new(response);
                HttpResponse::Ok().json(view.response())
            },
            Err(_) => HttpResponse::InternalServerError()
                .json(json!({ "error": "Failed to fetch maxFunds" })),
        },
    }
}

#[get("/user_addresses")]
async fn user_addresses(redis_client: web::Data<Client>) -> impl Responder {
    let mut con = redis_client.get_connection().unwrap();

    let user_addresses_redis_value: Result<String, RedisError> = con.get("user_addresses");

    match user_addresses_redis_value {
        Ok(user_addresses_json) => {
            let user_addresses: Vec<String> = serde_json::from_str(&user_addresses_json).unwrap();
            HttpResponse::Ok().json(UserAddressesResponse::new(user_addresses).response())
        },
        Err(_) => match fetch_user_addresses().await {
            Ok(response) => {
                println!("Setting user addresses in redis");

                let serialized_response = serde_json::to_string(&response).unwrap();
                let _: () = con.set("user_addresses", serialized_response).unwrap();

                let view = UserAddressesResponse::new(response);
                HttpResponse::Ok().json(view.response())
            },
            Err(_) => HttpResponse::InternalServerError()
                .json(json!({ "error": "Failed to fetch user addresses" })),
        },
    }
}

#[get("/contract_address")]
async fn contract_address(redis_client: web::Data<Client>) -> impl Responder {
    println!("Reading contract address from redis");
    let mut con = redis_client.get_connection().unwrap();
    let address: Result<String, RedisError> = con.get("contract_address");
    match address {
        Ok(address) => HttpResponse::Ok().json(json!({ "contract_address": address})),
        Err(_) => HttpResponse::NotFound().json(json!({ "error": "No contract deployed" })),
    }
}
