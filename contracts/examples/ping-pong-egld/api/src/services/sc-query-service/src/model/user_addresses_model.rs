use api::services::interactor::basic_interact::ActixInteractor;
use std::io::Error;

pub async fn fetch_user_addresses() -> Result<String, Error> {
    let mut interactor = ActixInteractor::init().await;

    let response = interactor.user_addresses().await;
    // response: 0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1, 3af8d9c9423b2577c6252722c1d90212a4111f7203f9744f76fcfa1d0a310033
    Ok(response)
}