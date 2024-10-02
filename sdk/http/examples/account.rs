use multiversx_sdk::data::address::Address;
use multiversx_sdk_http::{GatewayHttpProxy, DEFAULT_USE_CHAIN_SIMULATOR, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let addr = Address::from_bech32_string(
        "erd1qqqqqqqqqqqqqpgqfzydqmdw7m2vazsp6u5p95yxz76t2p9rd8ss0zp9ts",
    )
    .unwrap();

    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string(), DEFAULT_USE_CHAIN_SIMULATOR);
    let account = blockchain.get_account(&addr).await.unwrap();

    println!("account: {account:#?}");
}