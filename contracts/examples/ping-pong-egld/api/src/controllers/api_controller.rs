use std::str::FromStr;

use reqwest::*;
use rocket::{get, local::asynchronous::Client, post, response::content::RawText};
use serde_json::*;

#[post("/tx/<tx_type>", data = "<body>")]
pub async fn tx(tx_type: &str, body: &str) -> Value {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://localhost:8002/{}", tx_type))
        .body(body.to_string())
        .send()
        .await
        .unwrap();

    res.json().await.unwrap()
}
