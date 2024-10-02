use multiversx_sdk::data::{address::Address, vm::VMQueryInput};
use multiversx_sdk_http::{GatewayHttpProxy, DEFAULT_USE_CHAIN_SIMULATOR, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string(), DEFAULT_USE_CHAIN_SIMULATOR);
    let sc_address = Address::from_bech32_string(
        "erd1qqqqqqqqqqqqqpgq5dvvkmka7sujfsx7cfmygnx0n7luv8k0d8sskpqcec",
    )
    .unwrap();
    let req = VMQueryInput {
        sc_address: sc_address.clone(),
        func_name: "empty".to_string(),
        args: vec![],
    };
    let result = blockchain.execute_vmquery(&req).await;
    println!("{result:#?}");
}
