use multiversx_sdk::{
    chain_core::std::Bech32Address, data::transaction::Transaction, utils::base64_encode,
};
use multiversx_sdk_http::{GatewayHttpProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let tx = Transaction {
        nonce: 1,
        value: "50".to_string(),
        receiver: Bech32Address::from_bech32_string(
            "erd1rh5ws22jxm9pe7dtvhfy6j3uttuupkepferdwtmslms5fydtrh5sx3xr8r".to_owned(),
        ),
        sender: Bech32Address::from_bech32_string(
            "erd1rh5ws22jxm9pe7dtvhfy6j3uttuupkepferdwtmslms5fydtrh5sx3xr8r".to_owned(),
        ),
        data: Some(base64_encode("hello")),
        chain_id: "1".to_string(),
        version: 1,
        options: 0,
        gas_limit: 0,
        gas_price: 0,
        signature: None,
    };

    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let cost = blockchain.request_transaction_cost(&tx).await.unwrap();

    println!("tx cost: {cost:#?}");

    assert_eq!(
        cost.tx_gas_units, 57500,
        "receive cost {}",
        cost.tx_gas_units
    );
}
