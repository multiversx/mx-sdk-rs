use api::services::interactor::basic_interact::ActixInteractor;

pub async fn ping(egld_amount: u128, sender: String, contract_address: String) -> String {
    let mut basic_interactor = ActixInteractor::init().await;

    basic_interactor
        .ping(sender, contract_address, egld_amount)
        .await
}
