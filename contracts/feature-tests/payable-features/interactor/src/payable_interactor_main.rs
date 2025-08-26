extern crate payable_interactor;

#[tokio::main]
pub async fn main() {
    payable_interactor::payable_features_cli().await;
}
