use std::str::FromStr;

use crate::model::ping_model;
use rocket::{get, post};
use serde_json::*;

#[post("/ping", data = "<body>")]
pub async fn ping(body: String) -> Value {
    let res = Value::from_str(&body).unwrap();
    let egld_amount = res["value"].as_u64().unwrap() as u128;

    let result_value_after_tx = ping_model::ping(egld_amount).await;

    let json = json!({
        "response": result_value_after_tx
    });

    json
}
