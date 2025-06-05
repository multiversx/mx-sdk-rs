use multiversx_sdk::data::sdk_address::SdkAddress;
use multiversx_sdk_http::{GatewayHttpProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let addr = SdkAddress::from_bech32_string(
        "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th".to_owned(),
    );

    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let account_storage = blockchain
        .get_account_storage_keys(&addr.0, &addr.1)
        .await
        .unwrap();

    assert!(!account_storage.is_empty());
    println!("Account Storage: {account_storage:#?}");
}
