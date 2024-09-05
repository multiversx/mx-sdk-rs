use api::services::interactor::basic_interact::ActixInteractor;

pub async fn pong(sender: String, contract_address: String) -> String {
    let mut basic_interactor = ActixInteractor::init().await;

    basic_interactor.pong(sender, contract_address).await
}
