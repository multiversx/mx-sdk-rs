extern crate lottery_interactor;

#[tokio::main]
pub async fn main() {
    lottery_interactor::lottery_cli().await;
}
