use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PongReqBody {
    pub sender: String,
    pub contract_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct PongResponse {
    response: String,
}

impl PongReqBody {
    pub fn get_tx_sending_values(&self) -> (String, String) {
        (self.sender.clone(), self.contract_address.clone())
    }
}

impl PongResponse {
    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn send(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
