use multiversx_sdk::data::sdk_address::SdkAddress;
use multiversx_sdk_http::{GatewayHttpProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let addr = SdkAddress::from_bech32_string(
        "erd1qqqqqqqqqqqqqpgqfzydqmdw7m2vazsp6u5p95yxz76t2p9rd8ss0zp9ts",
    )
    .unwrap();

    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let account = blockchain.get_account(&addr.0).await.unwrap();

    assert!(account.address.to_bech32_string().is_ok());
    println!("account: {account:#?}");
}
