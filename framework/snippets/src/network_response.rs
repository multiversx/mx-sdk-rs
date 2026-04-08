use crate::sdk::{
    data::transaction::{ApiLogs, ApiSmartContractResult, Events, TransactionOnNetwork},
    utils::base64_decode,
};
use multiversx_sc_scenario::{
    imports::{Address, ESDTSystemSCAddress, ReturnCode},
    multiversx_chain_vm::{crypto_functions::keccak256, types::H256},
    scenario_model::{Log, TxResponse, TxResponseStatus},
};

const SC_DEPLOY_PROCESSING_TYPE: &str = "SCDeployment";
const LOG_IDENTIFIER_SIGNAL_ERROR: &str = "signalError";

/// Creates a [`TxResponse`] from a [`TransactionOnNetwork`].
pub fn parse_tx_response(tx: TransactionOnNetwork, return_code: ReturnCode) -> TxResponse {
    let tx_error = process_signal_error(&tx, return_code);
    if !tx_error.is_success() {
        return TxResponse {
            tx_error,
            tx_hash: process_tx_hash(&tx),
            ..Default::default()
        };
    }

    process_success(&tx)
}

fn process_signal_error(tx: &TransactionOnNetwork, return_code: ReturnCode) -> TxResponseStatus {
    if let Some(event) = find_log(tx, LOG_IDENTIFIER_SIGNAL_ERROR) {
        if event.topics.len() >= 2 {
            let error_message = String::from_utf8(base64_decode(&event.topics[1])).expect(
                "Failed to decode base64-encoded error message from transaction event topic",
            );
            return TxResponseStatus::new(return_code, &error_message);
        }
    }

    TxResponseStatus::default()
}

fn process_success(tx: &TransactionOnNetwork) -> TxResponse {
    TxResponse {
        out: process_out(tx),
        new_deployed_address: process_new_deployed_address(tx),
        new_issued_token_identifier: process_new_issued_token_identifier(tx),
        logs: process_logs(tx),
        tx_hash: process_tx_hash(tx),
        gas_used: tx.gas_used,
        ..Default::default()
    }
}

fn process_tx_hash(tx: &TransactionOnNetwork) -> Option<H256> {
    tx.hash.as_ref().map(|encoded_hash| {
        let decoded = hex::decode(encoded_hash).expect("error decoding tx hash from hex");
        assert_eq!(decoded.len(), 32);
        H256::from_slice(&decoded)
    })
}

fn process_out(tx: &TransactionOnNetwork) -> Vec<Vec<u8>> {
    let out_multi_transfer = tx.smart_contract_results.iter().find(is_multi_transfer);
    let out_scr = tx.smart_contract_results.iter().find(is_out_scr);

    if let Some(out_multi_transfer) = out_multi_transfer {
        log::trace!("Parsing result from multi transfer: {out_multi_transfer:?}");
        if let Some(data) = decode_multi_transfer_data_or_panic(out_multi_transfer.logs.clone()) {
            return data;
        }
    }

    if let Some(out_scr) = out_scr {
        log::trace!("Parsing result from scr: {out_scr:?}");
        return decode_scr_data_or_panic(&out_scr.data);
    }

    log::trace!("Parsing result from logs");
    process_out_from_log(tx).unwrap_or_default()
}

fn process_logs(tx: &TransactionOnNetwork) -> Vec<Log> {
    if let Some(api_logs) = &tx.logs {
        return api_logs
            .events
            .iter()
            .map(|event| Log {
                address: event.address.address.clone(),
                endpoint: event.identifier.clone(),
                topics: extract_topics(event),
                data: extract_data(event),
            })
            .collect::<Vec<Log>>();
    }

    Vec::new()
}

fn extract_data(event: &Events) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    event
        .data
        .for_each(|data_field| out.push(data_field.clone().into_bytes()));
    out
}

fn extract_topics(event: &Events) -> Vec<Vec<u8>> {
    event
        .topics
        .clone()
        .into_iter()
        .map(|s| s.into_bytes())
        .collect()
}

fn process_out_from_log(tx: &TransactionOnNetwork) -> Option<Vec<Vec<u8>>> {
    if let Some(logs) = &tx.logs {
        logs.events.iter().rev().find_map(|event| {
            if event.identifier == "writeLog" {
                let out = extract_write_log_data(event);
                return Some(out);
            }

            None
        })
    } else {
        None
    }
}

fn process_new_deployed_address(tx: &TransactionOnNetwork) -> Option<Address> {
    if tx.processing_type_on_destination != SC_DEPLOY_PROCESSING_TYPE {
        return None;
    }

    let sender_address_bytes = tx.sender.address.as_bytes();
    let sender_nonce_bytes = tx.nonce.to_le_bytes();
    let mut bytes_to_hash: Vec<u8> = Vec::new();
    bytes_to_hash.extend_from_slice(sender_address_bytes);
    bytes_to_hash.extend_from_slice(&sender_nonce_bytes);

    let address_keccak = keccak256(&bytes_to_hash);

    let mut address = [0u8; 32];

    address[0..8].copy_from_slice(&[0u8; 8]);
    address[8..10].copy_from_slice(&[5, 0]);
    address[10..30].copy_from_slice(&address_keccak[10..30]);
    address[30..32].copy_from_slice(&sender_address_bytes[30..32]);

    Some(Address::from(address))
}

fn process_new_issued_token_identifier(tx: &TransactionOnNetwork) -> Option<String> {
    let original_tx_data = String::from_utf8(base64_decode(tx.data.as_ref().unwrap())).unwrap();

    for scr in tx.smart_contract_results.iter() {
        if scr.sender.address != ESDTSystemSCAddress.to_address() {
            continue;
        }

        let prev_tx_data: &str = if let Some(prev_tx) = tx
            .smart_contract_results
            .iter()
            .find(|e| e.hash == scr.prev_tx_hash)
        {
            prev_tx.data.as_ref()
        } else if &scr.prev_tx_hash == tx.hash.as_ref().unwrap() {
            &original_tx_data
        } else {
            continue;
        };

        let is_issue_fungible = prev_tx_data.starts_with("issue@");
        let is_issue_semi_fungible = prev_tx_data.starts_with("issueSemiFungible@");
        let is_issue_non_fungible = prev_tx_data.starts_with("issueNonFungible@");
        let is_register_meta_esdt = prev_tx_data.starts_with("registerMetaESDT@");
        let is_register_and_set_all_roles_esdt =
            prev_tx_data.starts_with("registerAndSetAllRoles@");
        let is_register_dynamic_esdt = prev_tx_data.starts_with("registerDynamic");
        let is_register_and_set_all_roles_dynamic_esdt =
            prev_tx_data.starts_with("registerAndSetAllRolesDynamic@");

        if !is_issue_fungible
            && !is_issue_semi_fungible
            && !is_issue_non_fungible
            && !is_register_meta_esdt
            && !is_register_and_set_all_roles_esdt
            && !is_register_dynamic_esdt
            && !is_register_and_set_all_roles_dynamic_esdt
        {
            continue;
        }

        if scr.data.starts_with("ESDTTransfer@") {
            let encoded_tid = scr.data.split('@').nth(1);
            return Some(String::from_utf8(hex::decode(encoded_tid?).unwrap()).unwrap());
        } else if scr.data.starts_with("@00@") || scr.data.starts_with("@6f6b@") {
            let encoded_tid = scr.data.split('@').nth(2);
            return Some(String::from_utf8(hex::decode(encoded_tid?).unwrap()).unwrap());
        }
    }
    None
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

/// Decodes the data of a multi transfer result.
pub fn decode_multi_transfer_data_or_panic(logs: Option<ApiLogs>) -> Option<Vec<Vec<u8>>> {
    let logs = logs?;

    if let Some(event) = logs
        .events
        .iter()
        .find(|event| event.identifier == "writeLog")
    {
        let out = extract_write_log_data(event);
        return Some(out);
    }

    None
}

fn extract_write_log_data(event: &Events) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    event.data.for_each(|data_member| {
        let decoded_data = String::from_utf8(base64_decode(data_member)).unwrap();

        if decoded_data.starts_with('@') {
            let out_content = decode_scr_data_or_panic(decoded_data.as_str());
            out.extend(out_content);
        }
    });

    out
}

/// Checks if the given smart contract result is an out smart contract result.
pub fn is_out_scr(scr: &&ApiSmartContractResult) -> bool {
    scr.nonce != 0 && scr.data.starts_with('@')
}

/// Checks if the given smart contract result is a multi transfer smart contract result.
pub fn is_multi_transfer(scr: &&ApiSmartContractResult) -> bool {
    scr.data.starts_with("MultiESDTNFTTransfer@")
}
