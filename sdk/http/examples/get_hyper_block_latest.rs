use multiversx_sdk_http::{GatewayHttpProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let result = blockchain.get_latest_hyper_block_nonce().await;

    println!("latest block result: {result:?}")
}
