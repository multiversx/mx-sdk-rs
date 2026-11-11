extern crate adder_interactor;

#[tokio::main]
pub async fn main() {
    adder_interactor::adder_cli().await;
}
