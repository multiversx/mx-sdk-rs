use multiversx_sdk::gateway::{GatewayProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = GatewayProxy::new(DEVNET_GATEWAY.to_string());
    let result = blockchain.get_latest_hyper_block_nonce(false).await;

    println!("latest block result: {result:?}")
}
