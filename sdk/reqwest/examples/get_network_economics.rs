use multiversx_sdk_reqwest::gateway::{GatewayProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = GatewayProxy::new(DEVNET_GATEWAY.to_string());
    let network_economics = blockchain.get_network_economics().await.unwrap();

    println!("network_economics: {network_economics:#?}")
}
