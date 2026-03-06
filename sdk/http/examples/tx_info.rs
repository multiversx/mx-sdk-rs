use multiversx_sdk_http::*;

#[tokio::main]
async fn main() {
    let tx_hash = "dd810b6daeed111d5425cdbb47e2b125694580012c8682d155117d2967a549cb";
    let blockchain = GatewayHttpProxy::new(MAINNET_GATEWAY.to_string());

    let status = blockchain.get_transaction_status(tx_hash).await;
    assert!(status.is_ok());
    println!("tx status: {status:?}");

    let tx = blockchain.get_transaction_info(tx_hash).await;
    assert!(tx.is_ok());
    println!("tx: {tx:#?}");

    let tx = blockchain.get_transaction_info_with_results(tx_hash).await;
    assert!(tx.is_ok());
    println!("tx with results: {tx:#?}");
}
