use multiversx_sdk::{
    data::{address::Address, vm::VmValueRequest},
    gateway::{GatewayProxy, DEVNET_GATEWAY},
    wallet::Wallet,
};

#[tokio::main]
async fn main() {
    let wl = Wallet::from_private_key(
        "1648ad209d6b157a289884933e3bb30f161ec7113221ec16f87c3578b05830b0",
    )
    .unwrap();
    let addr = wl.address();
    let blockchain = GatewayProxy::new(DEVNET_GATEWAY.to_string());
    let req = VmValueRequest {
        sc_address: Address::from_bech32_string(
            "erd1qqqqqqqqqqqqqpgqhn3ae8dpc957t7jadn7kywtg503dy7pnj9ts3umqxx",
        )
        .unwrap(),
        func_name: "get".to_string(),
        args: vec![],
        caller: addr.clone(),
        value: "0".to_string(),
    };
    let result = blockchain.execute_vmquery(&req).await;
    println!("{result:#?}");
}
