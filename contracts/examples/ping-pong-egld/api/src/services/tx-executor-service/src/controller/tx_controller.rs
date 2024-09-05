use crate::{
    model::{deploy_model, ping_model, pong_model},
    view::{
        deploy_view::{DeployReqBody, DeployResponse},
        ping_view::{PingReqBody, PingResponse},
        pong_view::{PongReqBody, PongResponse},
    },
};
use actix_web::{web, Responder};

pub async fn ping(body: web::Json<PingReqBody>) -> impl Responder {
    let (value, sender, contract_address) = body.get_tx_sending_values();
    PingResponse::new(ping_model::ping(value, sender, contract_address).await).send()
}

pub async fn deploy(body: web::Json<DeployReqBody>) -> impl Responder {
    let (ping_amount, max_funds, activation_timestamp, duration, deployer) =
        body.get_tx_sending_values();

    DeployResponse::new(
        deploy_model::deploy(
            ping_amount,
            max_funds,
            activation_timestamp,
            duration,
            deployer,
        )
        .await,
    )
    .send()
}

pub async fn pong(body: web::Json<PongReqBody>) -> impl Responder {
    let (sender, contract_address) = body.get_tx_sending_values();

    PongResponse::new(pong_model::pong(sender, contract_address).await).send()
}
