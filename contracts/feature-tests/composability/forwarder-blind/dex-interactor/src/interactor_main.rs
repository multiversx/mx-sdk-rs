extern crate forwarder_blind_dex_interactor;

#[tokio::main]
pub async fn main() {
    forwarder_blind_dex_interactor::forwarder_blind_cli().await;
}
