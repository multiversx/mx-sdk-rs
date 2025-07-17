extern crate governance_sc_interact;

#[tokio::main]
pub async fn main() {
    governance_sc_interact::governance_sc_interact_cli().await;
}
