use mandos::model::{CheckLogs, Checkable, TxExpect};

use crate::{address_hex, bytes_to_string, tx_mock::TxResult, verbose_hex};

pub fn check_tx_output(tx_id: &str, tx_expect: &TxExpect, tx_result: &TxResult) {
    let have_str = tx_result.result_message.as_str();
    assert!(
        tx_expect.status.check(tx_result.result_status),
        "result code mismatch. Tx id: {}. Want: {}. Have: {}. Message: {}",
        tx_id,
        tx_expect.status,
        tx_result.result_status,
        have_str,
    );

    assert_eq!(
        tx_expect.out.len(),
        tx_result.result_values.len(),
        "bad out value. Tx id: {}. Want: {:?}. Have: {:?}",
        tx_id,
        tx_expect.out,
        tx_result.result_values
    );
    for (i, expected_out) in tx_expect.out.iter().enumerate() {
        let actual_value = &tx_result.result_values[i];
        assert!(
            expected_out.check(actual_value.as_slice()),
            "bad out value. Tx id: {}. Want: {}. Have: {}",
            tx_id,
            expected_out,
            verbose_hex(actual_value.as_slice())
        );
    }

    assert!(
        tx_expect.message.check(tx_result.result_message.as_bytes()),
        "result message mismatch. Tx id: {}. Want: {}. Have: {}.",
        tx_id,
        &tx_expect.message,
        have_str,
    );

    match &tx_expect.logs {
        CheckLogs::Star => {},
        CheckLogs::List(expected_logs) => {
            assert!(
                expected_logs.len() == tx_result.result_logs.len(),
                "Log amounts do not match. Tx id: {}. Want: {}. Have: {}",
                tx_id,
                expected_logs.len(),
                tx_result.result_logs.len()
            );

            for (expected_log, actual_log) in expected_logs.iter().zip(tx_result.result_logs.iter())
            {
                assert!(
					actual_log.equals(expected_log),
					"Logs do not match. Tx id: {}.\nWant: Address: {}, Identifier: {}, Topics: {:?}, Data: {}\nHave: Address: {}, Identifier: {}, Topics: {:?}, Data: {}",
					tx_id,
					verbose_hex(&expected_log.address.value),
					bytes_to_string(&expected_log.endpoint.value),
					expected_log.topics.iter().map(|topic| verbose_hex(&topic.value)).collect::<String>(),
					verbose_hex(&expected_log.data.value),
					address_hex(&actual_log.address),
					bytes_to_string(&actual_log.endpoint),
					actual_log.topics.iter().map(|topic| verbose_hex(topic)).collect::<String>(),
					verbose_hex(&actual_log.data),
				);
            }
        },
    }
}
