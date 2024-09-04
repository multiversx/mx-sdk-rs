use std::str::FromStr;

use api::services::interactor::basic_interact::ActixInteractor;
use rocket::{get, post};
use serde_json::*;

#[post("/ping", data = "<body>")]
pub async fn ping(body: String) -> Value {
    let res = Value::from_str(&body).unwrap();
    println!("{}", res);
    let json = json!({
        "response": res["value"].as_u64().unwrap()
    });

    json
}
