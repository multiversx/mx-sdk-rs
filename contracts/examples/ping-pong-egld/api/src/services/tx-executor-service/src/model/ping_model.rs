use api::services::interactor::basic_interact::ActixInteractor;

pub async fn ping(egld_amount: u128, sender: String) -> String {
    let mut basic_interactor = ActixInteractor::init().await;

    basic_interactor.ping(sender, egld_amount).await
}
