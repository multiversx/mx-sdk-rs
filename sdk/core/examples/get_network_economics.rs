use multiversx_sdk::blockchain::{CommunicationProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = CommunicationProxy::new(DEVNET_GATEWAY.to_string());
    let network_economics = blockchain.get_network_economics().await.unwrap();

    println!("network_economics: {network_economics:#?}")
}
