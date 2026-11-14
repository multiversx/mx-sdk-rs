use std::fmt;

use multiversx_chain_vm::{
    display_util::{address_hex, verbose_hex_list},
    host::context::result_values_to_string,
};

use super::{CheckLog, CheckLogs, Log, TxExpect, TxResponse};
use crate::scenario_model::Checkable;

/// A transaction response did not match its [`TxExpect`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxExpectMismatch {
    /// What was checked: "status", "out", "message" or "logs".
    pub field: &'static str,
    /// Description of what went wrong, if more specific than "`field` mismatch".
    pub reason: Option<String>,
    pub expected: String,
    pub actual: String,
}

impl TxExpectMismatch {
    fn new(field: &'static str, expected: String, actual: String) -> Self {
        TxExpectMismatch {
            field,
            reason: None,
            expected,
            actual,
        }
    }

    fn logs(reason: String, expected: String, actual: String) -> Self {
        TxExpectMismatch {
            field: "logs",
            reason: Some(reason),
            expected,
            actual,
        }
    }
}

impl fmt::Display for TxExpectMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.reason {
            Some(reason) => write!(f, "{reason}")?,
            None => write!(f, "result {} mismatch", self.field)?,
        }
        write!(f, ". Want: {}. Have: {}", self.expected, self.actual)
    }
}

impl std::error::Error for TxExpectMismatch {}

impl TxExpect {
    /// Checks the response against this expectation: status, out, message and logs.
    ///
    /// Gas and refund are not checked. Returns the first mismatch found.
    pub fn check_response(&self, tx_response: &TxResponse) -> Result<(), TxExpectMismatch> {
        let status = &tx_response.tx_error.status;
        let message = &tx_response.tx_error.message;

        if !self.status.check(status.as_u64()) {
            return Err(TxExpectMismatch::new(
                "status",
                self.status.to_string(),
                format!("{status} (message: {message})"),
            ));
        }

        if !self.out.check(tx_response.out.as_slice()) {
            return Err(TxExpectMismatch::new(
                "out",
                format!("[{}]", self.out_to_string()),
                format!("[{}]", result_values_to_string(&tx_response.out)),
            ));
        }

        if !self.message.check(message.as_str()) {
            return Err(TxExpectMismatch::new(
                "message",
                self.message.to_string(),
                message.clone(),
            ));
        }

        check_logs(&self.logs, &tx_response.logs)
    }
}

fn check_logs(expected_logs: &CheckLogs, actual_logs: &[Log]) -> Result<(), TxExpectMismatch> {
    let CheckLogs::List(expected_logs) = expected_logs else {
        return Ok(());
    };

    if actual_logs.len() < expected_logs.list.len() {
        return Err(TxExpectMismatch::logs(
            "too few logs".to_string(),
            expected_logs.list.len().to_string(),
            actual_logs.len().to_string(),
        ));
    }

    for (index, actual_log) in actual_logs.iter().enumerate() {
        match expected_logs.list.get(index) {
            Some(expected_log) => {
                if let Err(reason) = check_log(actual_log, expected_log) {
                    return Err(TxExpectMismatch::logs(
                        format!("{reason}, log index {index}"),
                        expected_log_to_string(expected_log),
                        actual_log_to_string(actual_log),
                    ));
                }
            }
            None => {
                if !expected_logs.more_allowed_at_end {
                    return Err(TxExpectMismatch::logs(
                        format!("unexpected log, log index {index}"),
                        "no more logs".to_string(),
                        actual_log_to_string(actual_log),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn check_log(actual_log: &Log, expected_log: &CheckLog) -> Result<(), &'static str> {
    if !expected_log.address.check(actual_log.address.as_bytes()) {
        return Err("log address does not match");
    }
    if !expected_log.endpoint.check(actual_log.endpoint.as_str()) {
        return Err("log endpoint does not match");
    }
    if !expected_log.topics.check(actual_log.topics.as_slice()) {
        return Err("log topics do not match");
    }
    if !expected_log.data.check(actual_log.data.as_slice()) {
        return Err("log data do not match");
    }
    Ok(())
}

fn expected_log_to_string(expected_log: &CheckLog) -> String {
    format!(
        "Address: {}, Endpoint: {}, Topics: {:?}, Data: {:?}",
        expected_log.address,
        expected_log.endpoint,
        expected_log.topics.pretty_str(),
        expected_log.data.pretty_str(),
    )
}

fn actual_log_to_string(actual_log: &Log) -> String {
    format!(
        "Address: {}, Endpoint: {}, Topics: {}, Data: {}",
        address_hex(&actual_log.address),
        actual_log.endpoint,
        verbose_hex_list(&actual_log.topics),
        verbose_hex_list(&actual_log.data),
    )
}
