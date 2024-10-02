use multiversx_sdk::data::sdk_address::SdkAddress;
use multiversx_sdk_http::{GatewayHttpProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let addr = SdkAddress::from_bech32_string(
        "erd1pdv0h3ddqyzlraek02y5rhmjnwwapjyhqm983kfcdfzmr6axqhdsfg4akx",
    )
    .unwrap();

    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let balances = blockchain.get_account_esdt_tokens(&addr).await.unwrap();

    println!("{balances:#?}");
}
