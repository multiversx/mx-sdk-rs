use api::services::interactor::basic_interact::ActixInteractor;

pub async fn pong(sender: String) -> String {
    let mut basic_interactor = ActixInteractor::init().await;

    basic_interactor.pong(sender).await
}
