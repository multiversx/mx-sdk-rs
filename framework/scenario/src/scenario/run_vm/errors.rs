use multiversx_chain_vm::tx_mock::TxFunctionName;
use multiversx_sc::chain_core::types::ReturnCode;

use crate::scenario_model::{CheckValue, U64Value};

pub(crate) fn default_error(
    error: &str,
    tx_id: &str,
    want: &CheckValue<U64Value>,
    have: ReturnCode,
    message: &str,
) -> String {
    format!(
        r#"
    Error: {}
    Tx id: '{}'. 
    Want: {}. 
    Have: {}.
    Message: {}"#,
        error, tx_id, want, have, message
    )
}

pub(crate) fn error_no_message(error: &str, tx_id: &str, want: String, have: String) -> String {
    format!(
        r#"
    Error: {}
    Tx id: '{}'. 
    Want: {}. 
    Have: {}."#,
        error, tx_id, want, have
    )
}

pub(crate) fn unexpected_log(
    tx_id: &str,
    index: usize,
    address: String,
    endpoints: &TxFunctionName,
    topics: String,
    data: String,
) -> String {
    format!(
        r#"
    Error: unexpected log,
    Tx id: '{}'. 
    Index: {}.
    Address: {}. 
    Endpoints: {}. 
    Topics: {}. 
    Data: {}."#,
        tx_id, index, address, endpoints, topics, data,
    )
}
