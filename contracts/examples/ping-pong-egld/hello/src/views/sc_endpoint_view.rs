use crate::models::basic_interact::{self, RocketInteractor};
use rocket::response::content::RawText;

pub async fn ping() -> String {
    String::from("PING SUCCESS")
}
