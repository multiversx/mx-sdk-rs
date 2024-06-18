use multiversx_sc_scenario::{
    imports::{Address, ESDTSystemSCAddress},
    multiversx_chain_vm::crypto_functions::keccak256,
    scenario_model::{TxResponse, TxResponseStatus},
};
use multiversx_sdk::{
    data::transaction::{ApiSmartContractResult, Events, TransactionOnNetwork},
    utils::base64_decode,
};

const SC_DEPLOY_PROCESSING_TYPE: &str = "SCDeployment";
const LOG_IDENTIFIER_SIGNAL_ERROR: &str = "signalError";

/// Creates a [`TxResponse`] from a [`TransactionOnNetwork`].
pub fn parse_tx_response(tx: TransactionOnNetwork) -> TxResponse {
    let tx_error = process_signal_error(&tx);
    if !tx_error.is_success() {
        TxResponse {
            tx_error,
            ..Default::default()
        };
    }

    let mut response = TxResponse::default();
    process(&mut response, &tx);
    response
}

fn process_signal_error(tx: &TransactionOnNetwork) -> TxResponseStatus {
    if let Some(event) = find_log(tx, LOG_IDENTIFIER_SIGNAL_ERROR) {
        let topics = event.topics.as_ref();
        if let Some(error) = process_topics_error(topics) {
            return TxResponseStatus::signal_error(&error);
        }

        let error_raw = base64_decode(topics.unwrap().get(1).unwrap());
        let error = String::from_utf8(error_raw).unwrap();
        return TxResponseStatus::signal_error(&error);
    }

    TxResponseStatus::default()
}

fn process(tx_response: &mut TxResponse, tx: &TransactionOnNetwork) {
    process_out(tx_response, tx);
    process_new_deployed_address(
        tx_response,
        tx.sender.to_bytes(),
        tx.nonce,
        tx.processing_type_on_destination.clone(),
    );
    process_new_issued_token_identifier(tx_response, tx);
}

fn process_out(tx_response: &mut TxResponse, tx: &TransactionOnNetwork) {
    let out_scr = tx.smart_contract_results.iter().find(is_out_scr);

    if let Some(out_scr) = out_scr {
        tx_response.out = decode_scr_data_or_panic(&out_scr.data);
    } else if let Some(data) = process_out_from_log(tx) {
        tx_response.out = data
    }
}

fn process_out_from_log(tx: &TransactionOnNetwork) -> Option<Vec<Vec<u8>>> {
    if let Some(logs) = &tx.logs {
        logs.events.iter().rev().find_map(|event| {
            if event.identifier == "writeLog" {
                if let Some(data) = &event.data {
                    let decoded_data = String::from_utf8(base64_decode(data)).unwrap();

                    if decoded_data.starts_with('@') {
                        let out = decode_scr_data_or_panic(decoded_data.as_str());
                        return Some(out);
                    }
                }
            }

            None
        })
    } else {
        None
    }
}

fn process_new_deployed_address(
    tx_response: &mut TxResponse,
    sender_address_bytes: [u8; 32],
    nonce: u64,
    processing_type_on_destination: String,
) {
    if processing_type_on_destination != SC_DEPLOY_PROCESSING_TYPE {
        return;
    }

    let sender_nonce_bytes = nonce.to_le_bytes();
    let mut bytes_to_hash: Vec<u8> = Vec::new();
    bytes_to_hash.extend_from_slice(&sender_address_bytes);
    bytes_to_hash.extend_from_slice(&sender_nonce_bytes);

    let address_keccak = keccak256(&bytes_to_hash);

    let mut address = [0u8; 32];

    address[0..8].copy_from_slice(&[0u8; 8]);
    address[8..10].copy_from_slice(&[5, 0]);
    address[10..30].copy_from_slice(&address_keccak[10..30]);
    address[30..32].copy_from_slice(&sender_address_bytes[30..32]);

    tx_response.new_deployed_address = Some(Address::from(address));
}

fn process_new_issued_token_identifier(tx_response: &mut TxResponse, tx: &TransactionOnNetwork) {
    // let api_scrs = tx
    //     .smart_contract_results
    //     .as_ref()
    //     .expect("missing smart contract results");
    for scr in tx.smart_contract_results.iter() {
        if scr.sender.to_bech32_string().unwrap() != ESDTSystemSCAddress.to_bech32_string() {
            continue;
        }

        let Some(prev_tx) = tx
            .smart_contract_results
            .iter()
            .find(|e| e.hash == scr.prev_tx_hash)
        else {
            continue;
        };

        let is_issue_fungible = prev_tx.data.starts_with("issue@");
        let is_issue_semi_fungible = prev_tx.data.starts_with("issueSemiFungible@");
        let is_issue_non_fungible = prev_tx.data.starts_with("issueNonFungible@");
        let is_register_meta_esdt = prev_tx.data.starts_with("registerMetaESDT@");
        let is_register_and_set_all_roles_esdt =
            prev_tx.data.starts_with("registerAndSetAllRoles@");

        if !is_issue_fungible
            && !is_issue_semi_fungible
            && !is_issue_non_fungible
            && !is_register_meta_esdt
            && !is_register_and_set_all_roles_esdt
        {
            continue;
        }

        if scr.data.starts_with("ESDTTransfer@") {
            let encoded_tid = scr.data.split('@').nth(1);
            if encoded_tid.is_none() {
                return;
            }

            tx_response.new_issued_token_identifier =
                Some(String::from_utf8(hex::decode(encoded_tid.unwrap()).unwrap()).unwrap());

            break;
        } else if scr.data.starts_with("@00@") {
            let encoded_tid = scr.data.split('@').nth(2);
            if encoded_tid.is_none() {
                return;
            }

            tx_response.new_issued_token_identifier =
                Some(String::from_utf8(hex::decode(encoded_tid.unwrap()).unwrap()).unwrap());

            break;
        }
    }
}

fn find_log<'a>(tx: &'a TransactionOnNetwork, log_identifier: &str) -> Option<&'a Events> {
    if let Some(logs) = &tx.logs {
        logs.events
            .iter()
            .find(|event| event.identifier == log_identifier)
    } else {
        None
    }
}

/// Checks for invalid topics.
pub fn process_topics_error(topics: Option<&Vec<String>>) -> Option<String> {
    if topics.is_none() {
        return Some("missing topics".to_string());
    }

    let topics = topics.unwrap();
    if topics.len() != 2 {
        Some(format!(
            "expected to have 2 topics, found {} instead",
            topics.len()
        ))
    } else {
        None
    }
}

/// Decodes the data of a smart contract result.
pub fn decode_scr_data_or_panic(data: &str) -> Vec<Vec<u8>> {
    let mut split = data.split('@');
    let _ = split.next().expect("SCR data should start with '@'");
    let result_code = split.next().expect("missing result code");
    assert_eq!(result_code, "6f6b", "result code is not 'ok'");

    split
        .map(|encoded_arg| hex::decode(encoded_arg).expect("error hex-decoding result"))
        .collect()
}

/// Checks if the given smart contract result is an out smart contract result.
pub fn is_out_scr(scr: &&ApiSmartContractResult) -> bool {
    scr.nonce != 0 && scr.data.starts_with('@')
}
