use crate::scenario::model::{Log, TxExpect, TxResponse, TxResponseStatus};

use multiversx_chain_vm::host::context::TxResult;

/// Checks a VM transaction result against the expectation, panics on mismatch.
///
/// Uses the same comparison as the interactors ([`TxExpect::check_response`]).
pub fn check_tx_output(tx_id: &str, tx_expect: &TxExpect, tx_result: &TxResult) {
    if let Err(mismatch) = tx_expect.check_response(&tx_result_to_response(tx_result)) {
        panic!(
            "{}{mismatch}. Tx id: '{tx_id}'",
            tx_expect.additional_error_message
        );
    }
}

/// Only the fields checked by [`TxExpect::check_response`]. Includes the ESDT transfer log.
fn tx_result_to_response(tx_result: &TxResult) -> TxResponse {
    TxResponse {
        out: tx_result.result_values.clone(),
        tx_error: TxResponseStatus {
            status: tx_result.result_status,
            message: tx_result.result_message.clone(),
        },
        logs: tx_result
            .all_logs()
            .into_iter()
            .map(|tx_log| Log {
                address: tx_log.address.clone(),
                endpoint: tx_log.endpoint.to_string(),
                topics: tx_log.topics.clone(),
                data: tx_log.data.clone(),
            })
            .collect(),
        ..Default::default()
    }
}
