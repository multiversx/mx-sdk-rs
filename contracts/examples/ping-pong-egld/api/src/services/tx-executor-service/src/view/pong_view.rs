use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PongReqBody {
    pub sender: String,
}

#[derive(Deserialize, Serialize)]
pub struct PongResponse {
    response: String,
}

impl PongReqBody {
    pub fn get_tx_sending_values(&self) -> String {
        self.sender.clone()
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
