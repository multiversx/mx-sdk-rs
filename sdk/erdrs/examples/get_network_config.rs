use mx_sdk_erdrs::blockchain::rpc::{ElrondProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    println!("network_config: {:#?}", network_config)
}
