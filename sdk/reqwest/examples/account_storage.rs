use multiversx_sdk::data::address::Address;
use multiversx_sdk_reqwest::gateway::{GatewayHttpProxy, DEFAULT_USE_CHAIN_SIMULATOR, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let addr = Address::from_bech32_string(
        "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
    )
    .unwrap();

    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string(), DEFAULT_USE_CHAIN_SIMULATOR);
    let account_storage = blockchain.get_account_storage_keys(&addr).await.unwrap();

    println!("Account Storage: {account_storage:#?}");
}
