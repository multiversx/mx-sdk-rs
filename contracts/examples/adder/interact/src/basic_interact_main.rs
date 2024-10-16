extern crate basic_interact;

#[tokio::main]
pub async fn main() {
    basic_interact::adder_cli().await;
}
