extern crate barnard_interactor;

#[tokio::main]
pub async fn main() {
    barnard_interactor::adder_cli().await;
}
