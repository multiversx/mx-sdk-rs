use multiversx_sdk::chain_core::std::Bech32Address;
use multiversx_sdk_http::{DEVNET_GATEWAY, GatewayHttpProxy};

#[tokio::main]
async fn main() {
    let addr = Bech32Address::from_bech32_string(
        "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th".to_owned(),
    );

    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let account_storage = blockchain.get_account_storage_keys(&addr).await.unwrap();

    assert!(!account_storage.is_empty());
    println!("Account Storage: {account_storage:#?}");
}
