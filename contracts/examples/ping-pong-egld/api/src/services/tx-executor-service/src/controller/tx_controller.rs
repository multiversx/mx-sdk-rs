use crate::{
    model::{deploy_model, ping_model, pong_model},
    view::{
        deploy_view::{DeployReqBody, DeployResponse},
        ping_view::{PingReqBody, PingResponse},
        pong_view::{PongReqBody, PongResponse},
    },
};
use actix_web::{web, Responder};
use redis::{Client, Commands};

pub async fn ping(body: web::Json<PingReqBody>, redis_client: web::Data<Client>) -> impl Responder {
    let (value, sender) = body.get_tx_sending_values();

    let response = ping_model::ping(value, sender).await;
    if response == "Tx successful" {
        let mut con = redis_client.get_connection().unwrap();
        let _: () = con.del("user_addresses").unwrap();
    }

    PingResponse::new(response).send()
}

pub async fn deploy(
    body: web::Json<DeployReqBody>,
    redis_client: web::Data<Client>,
) -> impl Responder {
    let (ping_amount, max_funds, activation_timestamp, duration, deployer) =
        body.get_tx_sending_values();

    let deploy_response = deploy_model::deploy(
        ping_amount,
        max_funds,
        activation_timestamp,
        duration,
        deployer,
    )
    .await;

    let mut con = redis_client.get_connection().unwrap();
    let _: () = con
        .set("contract_address", deploy_response.1.clone())
        .unwrap();
    let _: () = con.del("user_addresses").unwrap();
    let _: () = con.del("ping_amount").unwrap();
    let _: () = con.del("max_funds").unwrap();
    let _: () = con.del("deadline").unwrap();
    let _: () = con.del("timestamp").unwrap();

    DeployResponse::new(deploy_response).send()
}

pub async fn pong(body: web::Json<PongReqBody>) -> impl Responder {
    let sender = body.get_tx_sending_values();

    PongResponse::new(pong_model::pong(sender).await).send()
}
