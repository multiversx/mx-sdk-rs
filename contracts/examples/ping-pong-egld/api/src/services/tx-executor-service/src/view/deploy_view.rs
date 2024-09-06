use crate::view::helpers::denominate;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DeployReqBody {
    pub ping_amount: f64,
    pub max_funds: f64,
    pub activation_timestamp: String,
    pub duration: u64,
    pub deployer: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeployResponse {
    response: String,
    address: String,
}

impl DeployReqBody {
    pub fn get_tx_sending_values(&self) -> (u128, u128, String, u64, String) {
        (
            denominate(self.ping_amount),
            denominate(self.max_funds),
            self.activation_timestamp.clone(),
            self.duration,
            self.deployer.clone(),
        )
    }
}

impl DeployResponse {
    pub fn new(tx_response: (String, String)) -> Self {
        Self {
            response: tx_response.0,
            address: tx_response.1,
        }
    }

    pub fn send(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
