use multiversx_sdk_http::{DEVNET_GATEWAY, GatewayHttpProxy};

#[tokio::main]
async fn main() {
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let network_economics = blockchain.get_network_economics().await.unwrap();

    assert!(!network_economics.dev_rewards.is_empty());
    println!("network_economics: {network_economics:#?}")
}
