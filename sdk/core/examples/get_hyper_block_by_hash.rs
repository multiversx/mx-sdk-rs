use multiversx_sdk::blockchain::{CommunicationProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = CommunicationProxy::new(DEVNET_GATEWAY.to_string());
    let result = blockchain
        .get_hyper_block_by_hash("20b14ba0e68c465810c5ded091f220e51dad41629d7ccd87dab572206185e419")
        .await;

    println!("block by hash result: {result:?}");
}
