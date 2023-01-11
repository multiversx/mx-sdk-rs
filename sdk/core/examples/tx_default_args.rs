use multiversx_sdk::{
    blockchain::{CommunicationProxy, DEVNET_GATEWAY},
    data::address::Address,
};

#[tokio::main]
async fn main() {
    let blockchain = CommunicationProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();
    let addr = Address::from_bech32_string(
        "erd1qqqqqqqqqqqqqpgqfzydqmdw7m2vazsp6u5p95yxz76t2p9rd8ss0zp9ts",
    )
    .unwrap();

    let arg = blockchain
        .get_default_transaction_arguments(&addr, &network_config)
        .await
        .unwrap();

    println!("default tx arg: {arg:#?}");
}
