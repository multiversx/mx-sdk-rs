extern crate ping_pong_egld_interact;

#[tokio::main]
pub async fn main() {
    ping_pong_egld_interact::ping_pong_egld_cli().await;
}
