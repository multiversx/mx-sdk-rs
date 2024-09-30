use multiversx_sdk_reqwest::gateway::{GatewayProxy, DEFAULT_USE_CHAIN_SIMULATOR, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = GatewayProxy::new(DEVNET_GATEWAY.to_string(), DEFAULT_USE_CHAIN_SIMULATOR);
    let result = blockchain.get_hyper_block_by_nonce(7468).await;

    println!("block by nonce result: {result:#?}")
}
