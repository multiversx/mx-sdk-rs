use mx_sdk_erdrs::{
    blockchain::{ElrondProxy, DEVNET_GATEWAY},
    data::address::Address,
};

#[tokio::main]
async fn main() {
    let addr = Address::from_bech32_string(
        "erd1pdv0h3ddqyzlraek02y5rhmjnwwapjyhqm983kfcdfzmr6axqhdsfg4akx",
    )
    .unwrap();

    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let balances = blockchain.get_account_esdt_tokens(&addr).await.unwrap();

    println!("{:#?}", balances);
}
