use multiversx_sdk_http::{GatewayHttpProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    println!("network_config: {network_config:#?}")
}
