use crate::{
    data::transaction::{Events, TransactionOnNetwork},
    gateway::{GetTxInfo, GetTxProcessStatus},
    utils::base64_encode,
};
use log::info;
use multiversx_chain_core::types::ReturnCode;

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

                        info!(
                            "Transaction retrieved successfully, with status {}: {:#?}",
                            status, transaction_info_with_results
                        );
                        return (transaction_info_with_results, ReturnCode::Success);
                    },
                    "fail" => {
                        let (error_code, error_message) = parse_reason(&reason);
                        let mut tx_info_with_results: TransactionOnNetwork = proxy
                            .request(GetTxInfo::new(&tx_hash).with_results())
                            .await
                            .unwrap();
                        replace_with_error_message(&mut tx_info_with_results, error_message);

                        return (tx_info_with_results, error_code);
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
    (TransactionOnNetwork::default(), ReturnCode::UserError)
}

pub fn parse_reason(reason: &str) -> (ReturnCode, String) {
    if reason.is_empty() {
        return (ReturnCode::UserError, String::new());
    }

    let (code, mut message) = find_code_and_message(reason);

    match code {
        Some(return_code) => {
            if message.is_empty() {
                message = ReturnCode::message(return_code).to_owned();
            }

            (return_code, base64_encode(message))
        },
        None => {
            if message.is_empty() {
                message = extract_message_from_string_reason(reason);
            }

            let return_code = ReturnCode::from_message(&message).unwrap_or(ReturnCode::UserError);

            (return_code, base64_encode(message))
        },
    }
}

fn find_code_and_message(reason: &str) -> (Option<ReturnCode>, String) {
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

fn extract_message_from_string_reason(reason: &str) -> String {
    let contract_error: Vec<&str> = reason.split('[').filter(|part| !part.is_empty()).collect();
    if contract_error.len() == 3 {
        let message: Vec<&str> = contract_error[1].split(']').collect();
        return message[0].to_string();
    }

    return contract_error.last().unwrap_or(&"").split(']').collect();
}

fn replace_with_error_message(tx: &mut TransactionOnNetwork, error_message: String) {
    if error_message.is_empty() {
        return;
    }

    if let Some(event) = find_log(tx) {
        if let Some(event_topics) = event.topics.as_mut() {
            if event_topics.len() == 2 {
                event_topics[1] = error_message;
            }
        }
    }
}

fn find_log<'a>(tx: &'a mut TransactionOnNetwork) -> Option<&'a mut Events> {
    if let Some(logs) = tx.logs.as_mut() {
        logs.events
            .iter_mut()
            .find(|event| event.identifier == LOG_IDENTIFIER_SIGNAL_ERROR)
    } else {
        None
    }
}

#[test]
fn parse_reason_with_only_reason_test() {
    let reason = "@6f7574206f662066756e6473";
    let (code, message) = find_code_and_message(reason);
    assert!(code.is_none());
    assert_eq!("out of funds", message);

    let (code, message) = parse_reason(reason);
    assert_eq!(ReturnCode::OutOfFunds, code);
    assert_eq!("b3V0IG9mIGZ1bmRz", message);
}

#[test]
fn parse_empty_reason_test() {
    let (code, message) = parse_reason("");
    assert_eq!(ReturnCode::UserError, code);
    assert!(message.is_empty());
}

#[test]
fn parse_reason_test() {
    let reason = "@04@63616c6c56616c7565206e6f7420657175616c732077697468206261736549737375696e67436f7374@20248548f50a8fda29910e851c07c6c331a7f9e7784201ff2486be0934dbf612@bce4fac4ef67b79dbd2ce619b96fc7f51a3d36f68d5989b16b6fc1e47bde345d@ea64ae9803ff02ad12d496ab9e2838cc3ec3f9197973749610b4ccd4def8c1d1@00";

    let (code, message) = find_code_and_message(reason);
    assert_eq!(Some(ReturnCode::UserError), code);
    assert_eq!("callValue not equals with baseIssuingCost", message);

    let (code, message) = parse_reason(reason);
    assert_eq!(ReturnCode::UserError, code);
    assert_eq!(
        "Y2FsbFZhbHVlIG5vdCBlcXVhbHMgd2l0aCBiYXNlSXNzdWluZ0Nvc3Q=",
        message
    );
}

#[test]
fn parse_reason_sc_panic_test() {
    let reason = "\n\truntime.go:856 [error signalled by smartcontract] [compoundRewards]\n\truntime.go:856 [error signalled by smartcontract] [compoundRewards]\n\truntime.go:853 [Guild closing]";
    let (code, message) = find_code_and_message(reason);
    assert!(code.is_none());
    assert!(message.is_empty());

    let message = extract_message_from_string_reason(&reason);
    assert_eq!("Guild closing", message);

    let (code, message) = parse_reason(reason);
    assert_eq!(ReturnCode::UserError, code);
    assert_eq!("R3VpbGQgY2xvc2luZw==", message);
}

#[test]
fn parse_reason_invalid_contract_test() {
    let reason = "\n\truntime.go:831 [invalid contract code (not found)] [buyCards]";
    let (code, message) = find_code_and_message(reason);
    assert!(code.is_none());
    assert!(message.is_empty());

    let message = extract_message_from_string_reason(&reason);
    assert_eq!("invalid contract code (not found)", message);

    let (code, message) = parse_reason(reason);
    assert_eq!(ReturnCode::UserError, code);
    assert_eq!("aW52YWxpZCBjb250cmFjdCBjb2RlIChub3QgZm91bmQp", message);
}

#[test]
fn replace_logs_test() {
    let tx_str = r#"
   {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
      "nonce": 9785,
      "round": 5861933,
      "epoch": 2416,
      "value": "10000000000000000",
      "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
      "sender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
      "gasPrice": 1000000000,
      "gasLimit": 100000000,
      "gasUsed": 100000000,
      "data": "cmVnaXN0ZXJBbmRTZXRBbGxSb2xlc0A2OTZlNzQ2NTcyNjE2Mzc0NmY3MkA0OTRlNTQ1MkA0NjRlNDdA",
      "signature": "48cf58cc4064580cd5bdb48fa134d59f40ad4a8fed4de53fa29e160fa6e8dc438fbec234391f0668d65d99493f2d4f133691ee40be578e23680a923b503d1405",
      "sourceShard": 1,
      "destinationShard": 4294967295,
      "blockNonce": 5795888,
      "blockHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
      "notarizedAtSourceInMetaNonce": 5795888,
      "NotarizedAtSourceInMetaHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
      "notarizedAtDestinationInMetaNonce": 5795888,
      "notarizedAtDestinationInMetaHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
      "miniblockType": "TxBlock",
      "miniblockHash": "029a399205b1274c87f672f07dddef4a7e37c44ed63f3c022db7d06b2dad6079",
      "hyperblockNonce": 5795888,
      "hyperblockHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
      "timestamp": 1729171598,
      "smartContractResults": [
        {
          "hash": "76c83f623acd1ef2978353a5c2d04dc7fce727b1d48400215a3832e6c04c9dbe",
          "nonce": 9785,
          "value": 10000000000000000,
          "receiver": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@6f7574206f662066756e6473",
          "prevTxHash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
          "originalTxHash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
          "gasLimit": 0,
          "gasPrice": 0,
          "callType": 0,
          "returnMessage": "callValue not equals with baseIssuingCost",
          "originalSender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
          "operation": "transfer"
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
            "identifier": "signalError",
            "topics": [
              "ATlHLv9ohncamC8wg9pdQh8kwpGB5jiIIo3IHKYNaeE=",
              "Y2FsbFZhbHVlIG5vdCBlcXVhbHMgd2l0aCBiYXNlSXNzdWluZ0Nvc3Q="
            ],
            "data": "QDZmNzU3NDIwNmY2NjIwNjY3NTZlNjQ3Mw==",
            "additionalData": [
              "QDZmNzU3NDIwNmY2NjIwNjY3NTZlNjQ3Mw=="
            ]
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "registerAndSetAllRoles",
      "initiallyPaidFee": "1138600000000000",
      "fee": "1138600000000000",
      "chainID": "D",
      "version": 1,
      "options": 0
    }
"#;

    let expected_tx_str = r#"
    {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
      "nonce": 9785,
      "round": 5861933,
      "epoch": 2416,
      "value": "10000000000000000",
      "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
      "sender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
      "gasPrice": 1000000000,
      "gasLimit": 100000000,
      "gasUsed": 100000000,
      "data": "cmVnaXN0ZXJBbmRTZXRBbGxSb2xlc0A2OTZlNzQ2NTcyNjE2Mzc0NmY3MkA0OTRlNTQ1MkA0NjRlNDdA",
      "signature": "48cf58cc4064580cd5bdb48fa134d59f40ad4a8fed4de53fa29e160fa6e8dc438fbec234391f0668d65d99493f2d4f133691ee40be578e23680a923b503d1405",
      "sourceShard": 1,
      "destinationShard": 4294967295,
      "blockNonce": 5795888,
      "blockHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
      "notarizedAtSourceInMetaNonce": 5795888,
      "NotarizedAtSourceInMetaHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
      "notarizedAtDestinationInMetaNonce": 5795888,
      "notarizedAtDestinationInMetaHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
      "miniblockType": "TxBlock",
      "miniblockHash": "029a399205b1274c87f672f07dddef4a7e37c44ed63f3c022db7d06b2dad6079",
      "hyperblockNonce": 5795888,
      "hyperblockHash": "ca932566b72843ac7c0ff43c7e3d570d6259ea16d217a51016633ab019854b7d",
      "timestamp": 1729171598,
      "smartContractResults": [
        {
          "hash": "76c83f623acd1ef2978353a5c2d04dc7fce727b1d48400215a3832e6c04c9dbe",
          "nonce": 9785,
          "value": 10000000000000000,
          "receiver": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@6f7574206f662066756e6473",
          "prevTxHash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
          "originalTxHash": "8a0765cb16dad91f34cbd266d30d760e2dcea79d6c6cfcb6f7ea2b36680c5a1b",
          "gasLimit": 0,
          "gasPrice": 0,
          "callType": 0,
          "returnMessage": "callValue not equals with baseIssuingCost",
          "originalSender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
          "operation": "transfer"
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
            "identifier": "signalError",
            "topics": [
              "ATlHLv9ohncamC8wg9pdQh8kwpGB5jiIIo3IHKYNaeE=",
              "b3V0IG9mIGZ1bmRz"
            ],
            "data": "QDZmNzU3NDIwNmY2NjIwNjY3NTZlNjQ3Mw==",
            "additionalData": [
              "QDZmNzU3NDIwNmY2NjIwNjY3NTZlNjQ3Mw=="
            ]
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "registerAndSetAllRoles",
      "initiallyPaidFee": "1138600000000000",
      "fee": "1138600000000000",
      "chainID": "D",
      "version": 1,
      "options": 0
    }
    "#;

    let mut tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(&tx_str).unwrap();
    let expected_tx: TransactionOnNetwork =
        serde_json::from_str::<TransactionOnNetwork>(&expected_tx_str).unwrap();

    replace_with_error_message(&mut tx, "b3V0IG9mIGZ1bmRz".to_owned());
    assert_eq!(
        expected_tx.logs.unwrap().events[0].topics,
        tx.logs.unwrap().events[0].topics
    );
}
