use multiversx_sdk_http::{GatewayHttpProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let tx_hash = "fd21782ddb9e2217a3239e849e39d1d2c8fa74142a73f2dda3adb3028c0514e9";
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());

    let status = blockchain.get_transaction_status(tx_hash).await;
    println!("tx status: {status:?}");

    let tx = blockchain.get_transaction_info(tx_hash).await;
    println!("tx: {tx:#?}");

    let tx = blockchain.get_transaction_info_with_results(tx_hash).await;
    println!("tx with results: {tx:#?}");
}
