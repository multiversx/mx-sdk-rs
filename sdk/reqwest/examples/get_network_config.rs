use multiversx_sdk_reqwest::gateway::{GatewayHttpProxy, DEFAULT_USE_CHAIN_SIMULATOR, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string(), DEFAULT_USE_CHAIN_SIMULATOR);
    let network_config = blockchain.get_network_config().await.unwrap();

    println!("network_config: {network_config:#?}")
}
