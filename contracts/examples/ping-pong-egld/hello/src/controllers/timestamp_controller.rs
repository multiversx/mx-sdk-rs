use crate::models::basic_interact::RocketInteractor;
use crate::views::timestamp_view::timestamp_view;
use rocket::{get, response::content::RawText};

#[get("/timestamp")]
pub async fn timestamp() -> RawText<String> {
    // let res = get_res().await;
    // timestamp_view(String::from("Hello"))
    RawText(String::from("Hello"))
}

pub async fn get_res() -> String {
    let mut basic_interact = RocketInteractor::init().await;

    basic_interact.activation_timestamp().await
}
