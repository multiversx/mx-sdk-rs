use mx_sdk_erdrs::blockchain::rpc::{ElrondProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let network_economics = blockchain.get_network_economics().await.unwrap();

    println!("network_economics: {:#?}", network_economics)
}
