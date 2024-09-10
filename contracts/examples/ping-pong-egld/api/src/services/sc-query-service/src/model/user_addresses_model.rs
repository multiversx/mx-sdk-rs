use api::services::interactor::basic_interact::ActixInteractor;
use std::io::Error;

pub async fn fetch_user_addresses() -> Result<Vec<String>, Error> {
    let mut interactor = ActixInteractor::init().await;

    let response = interactor.user_addresses().await;

    // Ok(response.iter().map(|address| address.to_string()).collect())

    Ok(response)
}
