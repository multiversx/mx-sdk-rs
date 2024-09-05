use api::services::interactor::basic_interact::ActixInteractor;

pub async fn deploy(
    ping_amount: u128,
    max_funds: u128,
    activation_timestamp: String,
    duration: u64,
    deployer: String,
) -> (String, String) {
    let mut basic_interactor = ActixInteractor::init().await;

    let contract_addr = basic_interactor
        .deploy(
            ping_amount,
            duration,
            match activation_timestamp.parse::<u64>() {
                Ok(timestamp) => Some(timestamp),
                Err(_) => None,
            },
            max_funds,
            deployer,
        )
        .await;

    ("Tx successful".to_string(), contract_addr)
}
