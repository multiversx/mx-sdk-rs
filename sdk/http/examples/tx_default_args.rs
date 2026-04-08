use multiversx_sdk::chain_core::std::Bech32Address;
use multiversx_sdk_http::{DEVNET_GATEWAY, GatewayHttpProxy};

#[tokio::main]
async fn main() {
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();
    let addr = Bech32Address::from_bech32_string(
        "erd1qqqqqqqqqqqqqpgqfzydqmdw7m2vazsp6u5p95yxz76t2p9rd8ss0zp9ts".to_owned(),
    );

    let arg = blockchain
        .get_default_transaction_arguments(&addr.address, &network_config)
        .await
        .unwrap();

    let _ = arg.rcv_addr.to_bech32_string();
    println!("default tx arg: {arg:#?}");
}
