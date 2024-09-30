use multiversx_sdk::{
    data::{address::Address, vm::VmValueRequest},
    wallet::Wallet,
};
use multiversx_sdk_reqwest::gateway::{GatewayProxy, DEFAULT_USE_CHAIN_SIMULATOR, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let wl = Wallet::from_pem_file("sdk/core/tests/alice.pem").unwrap();
    let addr = wl.address();
    let blockchain = GatewayProxy::new(DEVNET_GATEWAY.to_string(), DEFAULT_USE_CHAIN_SIMULATOR);
    let req = VmValueRequest {
        sc_address: Address::from_bech32_string(
            "erd1qqqqqqqqqqqqqpgq5dvvkmka7sujfsx7cfmygnx0n7luv8k0d8sskpqcec",
        )
        .unwrap(),
        func_name: "empty".to_string(),
        args: vec![],
        caller: addr.clone(),
        value: "0".to_string(),
    };
    let result = blockchain.execute_vmquery(&req).await;
    println!("{result:#?}");
}
