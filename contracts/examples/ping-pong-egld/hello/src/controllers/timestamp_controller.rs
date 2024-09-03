use rocket::{get, response::content::RawText};
// use crate::models::basic_interact::RocketInteractor;
use crate::views::timestamp_view::timestamp_view;

#[get("/timestamp")]
pub async fn timestamp() -> RawText<String> {
    // let mut basic_interact = RocketInteractor::init().await;

    // Call the model method to get the activation timestamp
    // match basic_interact.activation_timestamp().await {
    //     Ok(response) => timestamp_view(response),
    //     Err(_) => RawText("I failed".to_string()), // Error handling
    // }
    timestamp_view(String::from("Hello"))
}
