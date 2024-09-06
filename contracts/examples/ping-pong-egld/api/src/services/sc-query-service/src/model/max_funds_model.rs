use api::services::interactor::basic_interact::ActixInteractor;
use std::io::Error;

pub async fn fetch_max_funds() -> Result<String, Error> {
    let mut interactor = ActixInteractor::init().await;

    let response = interactor.max_funds().await;

    Ok(response)
}