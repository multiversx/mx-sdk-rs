use crate::data::transaction::TransactionOnNetwork;
use log::info;
use std::time::{Duration, Instant};

use super::GatewayProxy;

const INITIAL_BACKOFF_DELAY: f32 = 1.4;
const MAX_RETRIES: usize = 8;
const MAX_BACKOFF_DELAY: Duration = Duration::from_secs(6);

impl GatewayProxy {
    /// Retrieves a transaction from the network.
    pub async fn retrieve_tx_on_network(&self, tx_hash: String) -> TransactionOnNetwork {
        let mut retries = 0;
        let mut backoff_delay = Duration::from_secs_f32(INITIAL_BACKOFF_DELAY);
        let start_time = Instant::now();

        loop {
            match self.get_transaction_process_status(&tx_hash).await {
                Ok((status, reason)) => {
                    // checks if transaction status is final
                    match status.as_str() {
                        "success" => {
                            // retrieve transaction info with results
                            let transaction_info_with_results = self
                                .get_transaction_info_with_results(&tx_hash)
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

                            match result {
                                Ok((code, err)) => {
                                    info!("Transaction failed. Code: {code}, message: {err}");
                                    panic!("Transaction failed. Code: {code}, message: {err}")
                                },
                                Err(err) => {
                                    info!("Reason parsing error for failed transaction: {err}");
                                    panic!("Reason parsing error for failed transaction: {err}")
                                },
                            }
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
                    tokio::time::sleep(backoff_time).await;
                    backoff_delay *= 2; // exponential backoff
                },
            }
        }

        // retries have been exhausted
        let elapsed_time = start_time.elapsed();
        println!(
            "Fetching transaction failed and retries exhausted, returning default transaction. Total elapsed time: {:?}",
            elapsed_time
        );
        TransactionOnNetwork::default()
    }
}

pub fn parse_reason(reason: &str) -> Result<(u64, String), String> {
    let parts: Vec<&str> = reason.split('@').collect();

    if parts.len() < 2 {
        return Err("Invalid reason format".to_string());
    }

    let error_code_hex = parts[1];
    let error_message_hex = parts[2];

    let error_code =
        u64::from_str_radix(error_code_hex, 16).expect("Failed to decode error code as u64");

    let error_message =
        String::from_utf8(hex::decode(error_message_hex).expect("Failed to decode error message"))
            .expect("Failed to decode error message as UTF-8");

    Ok((error_code, error_message))
}
