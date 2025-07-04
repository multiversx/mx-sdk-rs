extern crate basic_interactor;

#[tokio::main]
pub async fn main() {
    basic_interactor::adder_cli().await;
}
