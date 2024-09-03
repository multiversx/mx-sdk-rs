use crate::models::basic_interact::RocketInteractor;
use crate::views::sc_endpoint_view;
use rocket::{get, response::content::RawText};
use serde_json::*;

#[get("/ping")]
pub async fn ping() -> Value {
    let res = sc_endpoint_view::ping().await;
    let json = json!({
        "response": res,
    });

    json
}
