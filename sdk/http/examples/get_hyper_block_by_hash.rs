use multiversx_sdk_http::{DEVNET_GATEWAY, GatewayHttpProxy};

#[tokio::main]
async fn main() {
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let result = blockchain
        .get_hyper_block_by_hash("d59e0dc7d407b1175655357cb8056ec3bb77961192753cddda2fb700c6ce71c6")
        .await;

    assert!(result.is_ok());
    println!("block by hash result: {result:#?}");
}
