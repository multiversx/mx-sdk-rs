use mx_sdk_erdrs::blockchain::rpc::{ElrondProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let result = blockchain.get_latest_hyper_block_nonce(false).await;

    println!("latest block result: {:?}", result)
}
