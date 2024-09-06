use api::services::interactor::basic_interact::ActixInteractor;
use std::io::Error;

pub async fn fetch_ping_amount() -> Result<String, Error> {
    let mut interactor = ActixInteractor::init().await;

    let response = interactor.ping_amount().await;

    Ok(response)
}