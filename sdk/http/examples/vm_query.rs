use multiversx_sdk::data::{sdk_address::SdkAddress, vm::VMQueryInput};
use multiversx_sdk_http::{GatewayHttpProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let sc_address = SdkAddress::from_bech32_string(
        "erd1qqqqqqqqqqqqqpgq5dvvkmka7sujfsx7cfmygnx0n7luv8k0d8sskpqcec",
    )
    .unwrap();
    let req = VMQueryInput {
        sc_address: sc_address.clone(),
        func_name: "empty".to_string(),
        args: vec![],
    };
    let result = blockchain.execute_vmquery(&req).await;
    assert!(result.is_ok());
    println!("{result:#?}");
}
