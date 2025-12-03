extern crate tiny_interactor;

#[tokio::main]
pub async fn main() {
    tiny_interactor::tiny_interactor_cli().await;
}
