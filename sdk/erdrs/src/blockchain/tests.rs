use crate::{
    blockchain::rpc::{ElrondProxy, DEVNET_GATEWAY},
    data::{address::Address, transaction::Transaction, vm::VmValueRequest},
    interactors::wallet::Wallet,
};

#[ignore]
#[tokio::test]
async fn get_network_config() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    println!("network_config: {:?}", network_config)
}

#[ignore]
#[tokio::test]
async fn get_network_economics() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let network_economics = blockchain.get_network_economics().await.unwrap();

    println!("network_economics: {:?}", network_economics)
}

#[ignore]
#[tokio::test]
async fn get_hyper_block_by_hash() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let block = blockchain
        .get_hyper_block_by_hash("73259e8ea46343b7074d7183baa786e2d169eb632bc94b01a2e1dc22ed8ebe1c")
        .await
        .unwrap();

    println!("block: {:?}", block)
}

#[ignore]
#[tokio::test]
async fn get_hyper_block_by_nonce() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let block = blockchain.get_hyper_block_by_nonce(7468).await.unwrap();

    println!("block: {:?}", block)
}

#[ignore]
#[tokio::test]
async fn get_latest_hyper_block_nonce() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let block = blockchain
        .get_latest_hyper_block_nonce(false)
        .await
        .unwrap();

    println!("latest block: {:?}", block)
}

#[ignore]
#[tokio::test]
async fn request_transaction_cost() {
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

    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let cost = blockchain.request_transaction_cost(&tx).await.unwrap();

    assert_eq!(
        cost.tx_gas_units, 57500,
        "receive cost {}",
        cost.tx_gas_units
    );
}

#[ignore]
#[tokio::test]
async fn get_account() {
    let addr = Address::from_bech32_string(
        "erd1qqqqqqqqqqqqqpgqfzydqmdw7m2vazsp6u5p95yxz76t2p9rd8ss0zp9ts",
    )
    .unwrap();

    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let account = blockchain.get_account(&addr).await.unwrap();

    println!("account: {:?}", account);
}

#[ignore]
#[tokio::test]
async fn get_transaction_info() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let tx = blockchain
        .get_transaction_info("f0d92368280e9ec541a8d1821072b7f5c74f441e9221292889f69ed5ae84931d")
        .await
        .unwrap();

    println!("tx: {:?}", tx);
}

#[ignore]
#[tokio::test]
async fn get_transaction_info_with_results() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let tx = blockchain
        .get_transaction_info_with_results(
            "f0d92368280e9ec541a8d1821072b7f5c74f441e9221292889f69ed5ae84931d",
        )
        .await
        .unwrap();

    println!("tx with results: {:?}", tx);
    println!("logs: {:?}", tx.logs);
}

#[ignore]
#[tokio::test]
async fn get_transaction_status() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let status = blockchain
        .get_transaction_status("f0d92368280e9ec541a8d1821072b7f5c74f441e9221292889f69ed5ae84931d")
        .await
        .unwrap();

    println!("tx's status: {:?}", status);
}

#[ignore]
#[tokio::test]
async fn get_default_transaction_arguments() {
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();
    let addr = Address::from_bech32_string(
        "erd1qqqqqqqqqqqqqpgqfzydqmdw7m2vazsp6u5p95yxz76t2p9rd8ss0zp9ts",
    )
    .unwrap();

    let arg = blockchain
        .get_default_transaction_arguments(&addr, &network_config)
        .await
        .unwrap();

    println!("default tx arg: {:?}", arg);
}

#[ignore]
#[tokio::test]
async fn sign_tx() {
    let wl = Wallet::from_private_key(
        "1648ad209d6b157a289884933e3bb30f161ec7113221ec16f87c3578b05830b0",
    )
    .unwrap();
    let addr = wl.address();
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    let arg = blockchain
        .get_default_transaction_arguments(&addr, &network_config)
        .await
        .unwrap();

    let mut unsign_tx = Transaction {
        nonce: arg.nonce,
        value: "0".to_string(),
        receiver: addr.clone(),
        sender: addr.clone(),
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
    println!("tx_hash {}", tx_hash);
}

#[ignore]
#[tokio::test]
async fn sign_txs() {
    let wl = Wallet::from_private_key(
        "1648ad209d6b157a289884933e3bb30f161ec7113221ec16f87c3578b05830b0",
    )
    .unwrap();
    let addr = wl.address();
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    let arg = blockchain
        .get_default_transaction_arguments(&addr, &network_config)
        .await
        .unwrap();

    let mut unsign_tx = Transaction {
        nonce: arg.nonce,
        value: "1000000000000000000".to_string(),
        receiver: addr.clone(),
        sender: addr.clone(),
        gas_price: arg.gas_price,
        gas_limit: arg.gas_limit,
        data: arg.data,
        signature: None,
        chain_id: arg.chain_id,
        version: arg.version,
        options: arg.options,
    };

    let mut txs: Vec<Transaction> = vec![];

    let signature = wl.sign_tx(&unsign_tx);
    unsign_tx.signature = Some(hex::encode(signature));
    txs.push(unsign_tx.clone());

    unsign_tx.version = 2;
    unsign_tx.options = 1;
    unsign_tx.nonce += 1;

    let signature = wl.sign_tx(&unsign_tx);
    unsign_tx.signature = Some(hex::encode(signature));
    txs.push(unsign_tx.clone());

    let tx_hash = blockchain.send_transactions(&txs).await.unwrap();
    println!("tx_hashs {:?}", tx_hash);
}

#[ignore]
#[tokio::test]
async fn execute_vmquery() {
    let wl = Wallet::from_private_key(
        "1648ad209d6b157a289884933e3bb30f161ec7113221ec16f87c3578b05830b0",
    )
    .unwrap();
    let addr = wl.address();
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
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
    println!("{:?}", result);
}

#[ignore]
#[tokio::test]
async fn decode_address() {
    let addr = Address::from_bech32_string(
        "erd1qqqqqqqqqqqqqpgqyfjjn43spw7teklwtpz4x5waygq2mluyj9ts0mdwn6",
    )
    .unwrap();
    let encode = hex::encode(addr.to_bytes());
    println!("{:?}", encode);
}

#[ignore]
#[tokio::test]
async fn get_esdt_tokens() {
    let addr = Address::from_bech32_string(
        "erd1e7zw7v7ykt37tdpxn4ckfqgw8q800pfw0wvdp46jnsj059gxcv2sfuy3h8",
    )
    .unwrap();

    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let balances = blockchain.get_account_esdt_tokens(&addr).await.unwrap();

    println!("{:?}", balances);
}
