use crate::view::helpers::denominate;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PingReqBody {
    pub sender: String,
    pub value: f64,
}

#[derive(Deserialize, Serialize)]
pub struct PingResponse {
    response: String,
}

impl PingReqBody {
    pub fn get_tx_sending_values(&self) -> (u128, String) {
        (denominate(self.value), self.sender.clone())
    }
}

impl PingResponse {
    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn send(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
