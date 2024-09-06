use api::services::interactor::basic_interact::ActixInteractor;
use std::io::Error;

pub async fn fetch_deadline() -> Result<String, Error> {
    let mut interactor = ActixInteractor::init().await;

    let response = interactor.deadline().await;

    Ok(response)
}