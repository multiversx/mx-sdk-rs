use multiversx_sdk_reqwest::gateway::{GatewayProxy, DEFAULT_USE_CHAIN_SIMULATOR, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = GatewayProxy::new(DEVNET_GATEWAY.to_string(), DEFAULT_USE_CHAIN_SIMULATOR);
    let result = blockchain
        .get_hyper_block_by_hash("d59e0dc7d407b1175655357cb8056ec3bb77961192753cddda2fb700c6ce71c6")
        .await;

    println!("block by hash result: {result:#?}");
}
