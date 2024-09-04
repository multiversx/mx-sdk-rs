use api::services::interactor::basic_interact::ActixInteractor;
use std::io::Error;

pub async fn fetch_timestamp() -> Result<String, Error> {
    let mut interactor = ActixInteractor::init().await;
    
    interactor.activation_timestamp().await;

    let response = interactor.activation_timestamp().await;
    Ok(response)
}
