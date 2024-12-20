extern crate system_sc_interact;

#[tokio::main]
pub async fn main() {
    system_sc_interact::system_sc_interact_cli().await;
}
