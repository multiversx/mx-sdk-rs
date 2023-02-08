use crate::scenario::model::{CheckLogs, Checkable, TxExpect};

use multiversx_chain_vm::{
    display_util::{address_hex, verbose_hex, verbose_hex_list},
    tx_mock::{TxLog, TxResult},
};

pub fn check_tx_output(tx_id: &str, tx_expect: &TxExpect, tx_result: &TxResult) {
    let have_str = tx_result.result_message.as_str();
    assert!(
        tx_expect.status.check(tx_result.result_status),
        "result code mismatch. Tx id: '{}'. Want: {}. Have: {}. Message: {}",
        tx_id,
        tx_expect.status,
        tx_result.result_status,
        have_str,
    );

    assert!(
        tx_expect.out.check(tx_result.result_values.as_slice()),
        "bad out value. Tx id: '{}'. Want: [{}]. Have: [{}]",
        tx_id,
        tx_expect.out_to_string(),
        tx_result.result_values_to_string()
    );

    assert!(
        tx_expect.message.check(tx_result.result_message.as_bytes()),
        "result message mismatch. Tx id: '{}'. Want: {}. Have: {}.",
        tx_id,
        &tx_expect.message,
        have_str,
    );

    match &tx_expect.logs {
        CheckLogs::Star => {},
        CheckLogs::List(expected_logs) => {
            assert!(
                tx_result.result_logs.len() >= expected_logs.list.len(),
                "Too few logs. Tx id: '{}'. Want: {}. Have: {}",
                tx_id,
                expected_logs.list.len(),
                tx_result.result_logs.len()
            );

            for (i, actual_log) in tx_result.result_logs.iter().enumerate() {
                if i < expected_logs.list.len() {
                    let expected_log = &expected_logs.list[i];
                    assert!(
                        scenario_check(actual_log, expected_log),
                        "Logs do not match. Tx id: '{}'. Index: {}.\nWant: Address: {}, Endpoint: {}, Topics: {:?}, Data: {}\nHave: Address: {}, Endpoint: {}, Topics: {:?}, Data: {}",
                        tx_id,
                        i,
                        &expected_log.address,
                        &expected_log.endpoint,
                        &expected_log.topics.pretty_str(),
                        &expected_log.data,
                        address_hex(&actual_log.address),
                    &actual_log.endpoint,
                        verbose_hex_list(actual_log.topics.as_slice()),
                        verbose_hex(&actual_log.data),
                    );
                } else if !expected_logs.more_allowed_at_end {
                    panic!(
                        "Unexpected log. Tx id: '{}'. Index: {}.\nAddress: {}, Endpoint: {}, Topics: {:?}, Data: {}",
                        tx_id,
                        i,
                        address_hex(&actual_log.address),
                        &actual_log.endpoint,
                        verbose_hex_list(actual_log.topics.as_slice()),
                        verbose_hex(&actual_log.data),
                    )
                }
            }
        },
    }
}

fn scenario_check(tx_log: &TxLog, check_log: &crate::scenario::model::CheckLog) -> bool {
    check_log.address.check(tx_log.address.as_bytes())
        && check_log.endpoint.check(&tx_log.endpoint)
        && check_log.topics.check(tx_log.topics.as_slice())
        && check_log.data.check(tx_log.data.as_slice())
}
