extern crate validator_rewards_interactor;

#[tokio::main]
pub async fn main() {
    validator_rewards_interactor::validator_rewards_cli().await;
}
