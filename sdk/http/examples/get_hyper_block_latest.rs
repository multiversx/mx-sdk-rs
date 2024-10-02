use multiversx_sdk_http::{GatewayHttpProxy, DEFAULT_USE_CHAIN_SIMULATOR, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string(), DEFAULT_USE_CHAIN_SIMULATOR);
    let result = blockchain.get_latest_hyper_block_nonce().await;

    println!("latest block result: {result:?}")
}
