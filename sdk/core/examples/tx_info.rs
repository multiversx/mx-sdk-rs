use multiversx_sdk::blockchain::{CommunicationProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let tx_hash = "49edb289892a655a0e988b360c19326c21107f9696c6197b435667c6e8c6e1a3";
    let blockchain = CommunicationProxy::new(DEVNET_GATEWAY.to_string());

    let status = blockchain.get_transaction_status(tx_hash).await;
    println!("tx status: {status:?}");

    let tx = blockchain.get_transaction_info(tx_hash).await;
    println!("tx: {tx:#?}");

    let tx = blockchain.get_transaction_info_with_results(tx_hash).await;
    println!("tx with results: {tx:#?}");
}
