use crate::{
    data::transaction::TransactionOnNetwork,
    gateway::{GetTxInfo, GetTxProcessStatus},
};
use log::info;

use crate::gateway::GatewayAsyncService;

const INITIAL_BACKOFF_DELAY: u64 = 1400;
const MAX_RETRIES: usize = 8;
const MAX_BACKOFF_DELAY: u64 = 6000;

/// Retrieves a transaction from the network.
pub async fn retrieve_tx_on_network<GatewayProxy: GatewayAsyncService>(
    proxy: &GatewayProxy,
    tx_hash: String,
) -> TransactionOnNetwork {
    let mut retries = 0;
    let mut backoff_delay = INITIAL_BACKOFF_DELAY;
    let start_time = proxy.now();

    loop {
        match proxy.request(GetTxProcessStatus::new(&tx_hash)).await {
            Ok((status, reason)) => {
                // checks if transaction status is final
                match status.as_str() {
                    "success" => {
                        // retrieve transaction info with results
                        let transaction_info_with_results = proxy
                            .request(GetTxInfo::new(&tx_hash).with_results())
                            .await
                            .unwrap();

                        info!(
                            "Transaction retrieved successfully, with status {}: {:#?}",
                            status, transaction_info_with_results
                        );
                        return transaction_info_with_results;
                    },
                    "fail" => {
                        // status failed and no reason means invalid transaction
                        if reason.is_empty() {
                            info!("Transaction failed. Invalid transaction: {tx_hash}");
                            panic!("Transaction failed. Invalid transaction: {tx_hash}");
                        }

                        let result = parse_reason(&reason);
                        let transaction_info_with_results = proxy
                            .request(GetTxInfo::new(&tx_hash).with_results())
                            .await
                            .unwrap();

                        println!("Transaction failed: {}", result);

                        return transaction_info_with_results;
                    },
                    _ => {
                        continue;
                    },
                }
            },
            Err(err) => {
                retries += 1;
                if retries >= MAX_RETRIES {
                    info!("Transaction failed, max retries exceeded: {}", err);
                    println!("Transaction failed, max retries exceeded: {}", err);
                    break;
                }

                let backoff_time = backoff_delay.min(MAX_BACKOFF_DELAY);
                proxy.sleep(backoff_time).await;
                backoff_delay *= 2; // exponential backoff
            },
        }
    }

    // retries have been exhausted
    println!(
            "Fetching transaction failed and retries exhausted, returning default transaction. Total elapsed time: {:?}s",
            proxy.elapsed_seconds(&start_time)
        );
    TransactionOnNetwork::default()
}

pub fn parse_reason(reason: &str) -> String {
    let parts: Vec<&str> = reason.split('@').filter(|part| !part.is_empty()).collect();
    let mut responses: Vec<String> = Vec::new();
    for part in &parts {
        match u64::from_str_radix(part, 16) {
            Ok(error_code) => responses.push(error_code.to_string()),
            Err(_) => responses.push(
                String::from_utf8(hex::decode(part).expect("Failed to decode error message"))
                    .expect("Failed to decode error message as UTF-8"),
            ),
        }
    }

    responses.join(" ")
}
