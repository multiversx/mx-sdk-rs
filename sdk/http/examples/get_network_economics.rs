use multiversx_sdk_http::{GatewayHttpProxy, DEFAULT_USE_CHAIN_SIMULATOR, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string(), DEFAULT_USE_CHAIN_SIMULATOR);
    let network_economics = blockchain.get_network_economics().await.unwrap();

    println!("network_economics: {network_economics:#?}")
}
