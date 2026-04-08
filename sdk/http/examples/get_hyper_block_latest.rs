use multiversx_sdk_http::{DEVNET_GATEWAY, GatewayHttpProxy};

#[tokio::main]
async fn main() {
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let result = blockchain.get_latest_hyper_block_nonce().await;

    assert!(result.is_ok());
    println!("latest block result: {result:?}")
}
