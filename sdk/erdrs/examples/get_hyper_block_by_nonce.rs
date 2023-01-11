use mx_sdk_erdrs::blockchain::{ElrondProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let result = blockchain.get_hyper_block_by_nonce(7468).await;

    println!("block by nonce result: {:?}", result)
}
