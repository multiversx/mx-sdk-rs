extern crate payable_interactor;

#[tokio::main]
pub async fn main() {
    payable_interactor::adder_cli().await;
}
