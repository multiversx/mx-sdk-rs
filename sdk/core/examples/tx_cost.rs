use multiversx_sdk::{
    blockchain::{CommunicationProxy, DEVNET_GATEWAY},
    data::{address::Address, transaction::Transaction},
};

#[tokio::main]
async fn main() {
    let tx = Transaction {
        nonce: 1,
        value: "50".to_string(),
        receiver: Address::from_bech32_string(
            "erd1rh5ws22jxm9pe7dtvhfy6j3uttuupkepferdwtmslms5fydtrh5sx3xr8r",
        )
        .unwrap(),
        sender: Address::from_bech32_string(
            "erd1rh5ws22jxm9pe7dtvhfy6j3uttuupkepferdwtmslms5fydtrh5sx3xr8r",
        )
        .unwrap(),
        data: Some(base64::encode("hello")),
        chain_id: "1".to_string(),
        version: 1,
        options: 0,
        gas_limit: 0,
        gas_price: 0,
        signature: None,
    };

    let blockchain = CommunicationProxy::new(DEVNET_GATEWAY.to_string());
    let cost = blockchain.request_transaction_cost(&tx).await.unwrap();

    println!("tx cost: {cost:#?}");

    assert_eq!(
        cost.tx_gas_units, 57500,
        "receive cost {}",
        cost.tx_gas_units
    );
}
