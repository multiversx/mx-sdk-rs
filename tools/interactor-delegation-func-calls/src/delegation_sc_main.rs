extern crate delegation_sc_interact;

#[tokio::main]
pub async fn main() {
    delegation_sc_interact::delegation_sc_interact_cli().await;
}
