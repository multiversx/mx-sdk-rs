use multiversx_sdk::{
    data::{sdk_address::SdkAddress, transaction::Transaction},
    wallet::Wallet,
};
use multiversx_sdk_http::{GatewayHttpProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let wl = Wallet::from_private_key(
        "1648ad209d6b157a289884933e3bb30f161ec7113221ec16f87c3578b05830b0",
    )
    .unwrap();
    let addr = wl.to_address();
    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    let arg = blockchain
        .get_default_transaction_arguments(&addr, &network_config)
        .await
        .unwrap();

    let mut unsign_tx = Transaction {
        nonce: arg.nonce,
        value: "0".to_string(),
        receiver: SdkAddress(addr.clone()),
        sender: SdkAddress(addr.clone()),
        gas_price: arg.gas_price,
        gas_limit: arg.gas_limit,
        data: arg.data,
        signature: None,
        chain_id: arg.chain_id,
        version: arg.version,
        options: arg.options,
    };

    let signature = wl.sign_tx(&unsign_tx);
    unsign_tx.signature = Some(hex::encode(signature));
    let tx_hash = blockchain.send_transaction(&unsign_tx).await.unwrap();
    println!("tx_hash {tx_hash}");
}
