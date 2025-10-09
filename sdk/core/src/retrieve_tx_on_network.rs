use crate::{
    data::transaction::{ApiLogs, Events, LogData, TransactionOnNetwork},
    gateway::{GetTxInfo, GetTxProcessStatus},
    utils::base64_encode,
};
use log::info;
use multiversx_chain_core::{std::Bech32Address, types::ReturnCode};

use crate::gateway::GatewayAsyncService;

const INITIAL_BACKOFF_DELAY: u64 = 1400;
const MAX_RETRIES: usize = 8;
const MAX_BACKOFF_DELAY: u64 = 6000;
const LOG_IDENTIFIER_SIGNAL_ERROR: &str = "signalError";

/// Retrieves a transaction from the network.
pub async fn retrieve_tx_on_network<GatewayProxy: GatewayAsyncService>(
    proxy: &GatewayProxy,
    tx_hash: String,
) -> (TransactionOnNetwork, ReturnCode) {
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

                        log::info!("Transaction retrieved successfully, with status {status}",);
                        return (transaction_info_with_results, ReturnCode::Success);
                    }
                    "fail" => {
                        let (error_code, reason) = parse_reason(&reason);

                        let mut failed_transaction: TransactionOnNetwork = proxy
                            .request(GetTxInfo::new(&tx_hash).with_results())
                            .await
                            .unwrap();

                        replace_with_error_message(&mut failed_transaction, &reason);

                        log::error!(
                            "Transaction failed with error code: {} and message: {reason}",
                            error_code.as_u64()
                        );

                        return (failed_transaction, error_code);
                    }
                    _ => {
                        continue;
                    }
                }
            }
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
            }
        }
    }

    // retries have been exhausted
    println!(
            "Fetching transaction failed and retries exhausted, returning default transaction. Total elapsed time: {:?}s",
            proxy.elapsed_seconds(&start_time)
        );

    let error_message = ReturnCode::message(ReturnCode::NetworkTimeout);
    let failed_transaction: TransactionOnNetwork = create_tx_failed(error_message);

    (failed_transaction, ReturnCode::NetworkTimeout)
}

pub fn parse_reason(reason: &str) -> (ReturnCode, String) {
    if reason.is_empty() {
        return (ReturnCode::UserError, "invalid transaction".to_string());
    }

    let (code, mut message) = find_code_and_message(reason);

    match code {
        Some(return_code) => {
            if message.is_empty() {
                ReturnCode::message(return_code).clone_into(&mut message);
            }

            (return_code, message)
        }
        None => {
            if message.is_empty() {
                message = extract_message_from_string_reason(reason);
            }
            let return_code = ReturnCode::from_message(&message).unwrap_or(ReturnCode::UserError);

            (return_code, message)
        }
    }
}

pub fn find_code_and_message(reason: &str) -> (Option<ReturnCode>, String) {
    let mut error_code: Option<ReturnCode> = None;
    let mut error_message: String = String::new();
    let parts: Vec<&str> = reason.split('@').filter(|part| !part.is_empty()).collect();

    for part in &parts {
        if let Ok(code) = u64::from_str_radix(part, 16) {
            if error_code.is_none() {
                error_code = ReturnCode::from_u64(code);
            }
        } else if let Ok(hex_decode_error_message) = hex::decode(part) {
            if let Ok(str) = String::from_utf8(hex_decode_error_message.clone()) {
                error_message = str;
            }
        }
    }

    (error_code, error_message)
}

pub fn extract_message_from_string_reason(reason: &str) -> String {
    let contract_error: Vec<&str> = reason.split('[').filter(|part| !part.is_empty()).collect();
    if contract_error.len() == 3 {
        let message: Vec<&str> = contract_error[1].split(']').collect();
        return message[0].to_string();
    }

    contract_error.last().unwrap_or(&"").split(']').collect()
}

fn create_tx_failed(error_message: &str) -> TransactionOnNetwork {
    let mut failed_transaction_info = TransactionOnNetwork::default();

    let log: ApiLogs = ApiLogs {
        address: Bech32Address::zero_default_hrp(),
        events: vec![Events {
            address: Bech32Address::zero_default_hrp(),
            identifier: LOG_IDENTIFIER_SIGNAL_ERROR.to_string(),
            topics: vec![String::new(), base64_encode(error_message.as_bytes())],
            data: LogData::default(),
        }],
    };

    failed_transaction_info.logs = Some(log);

    failed_transaction_info
}

pub fn replace_with_error_message(tx: &mut TransactionOnNetwork, error_message: &str) {
    if error_message.is_empty() {
        return;
    }

    let error_message_encoded = base64_encode(error_message);

    if let Some(event) = find_log(tx) {
        if event.topics.len() >= 2 && event.topics[1] != error_message_encoded {
            event.topics[1] = error_message_encoded;
        }
    }
}

fn find_log(tx: &mut TransactionOnNetwork) -> Option<&mut Events> {
    if let Some(logs) = tx.logs.as_mut() {
        logs.events
            .iter_mut()
            .find(|event| event.identifier == LOG_IDENTIFIER_SIGNAL_ERROR)
    } else {
        None
    }
}
