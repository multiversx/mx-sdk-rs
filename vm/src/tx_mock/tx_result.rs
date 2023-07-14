use std::fmt;

use super::{AsyncCallTxData, TxLog, TxPanic, TxResultCalls};

#[derive(Clone, Debug)]
#[must_use]
pub struct TxResult {
    pub result_status: u64,
    pub result_message: String,
    pub result_values: Vec<Vec<u8>>,
    pub result_logs: Vec<TxLog>,

    /// Calls that need to be executed.
    ///
    /// Structure is emptied as soon as async calls are executed.
    pub pending_calls: TxResultCalls,

    /// All async calls launched from the tx (legacy async, promises, transfer-execute).
    ///
    /// Is never cleared of its contents.
    pub all_calls: Vec<AsyncCallTxData>,
}

impl Default for TxResult {
    fn default() -> Self {
        TxResult {
            result_status: 0,
            result_message: String::new(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
            pending_calls: TxResultCalls::empty(),
            all_calls: Vec::new(),
        }
    }
}

impl TxResult {
    pub fn empty() -> TxResult {
        TxResult::default()
    }

    pub fn print(&self) {
        println!("{self}");
    }

    pub fn from_panic_obj(panic_obj: &TxPanic) -> Self {
        TxResult {
            result_status: panic_obj.status,
            result_message: panic_obj.message.clone(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
            ..Default::default()
        }
    }

    pub fn from_panic_string(s: &str) -> Self {
        TxResult {
            result_status: 4,
            result_message: s.to_string(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
            ..Default::default()
        }
    }

    pub fn from_unknown_panic() -> Self {
        Self::from_panic_string("")
    }

    pub fn from_vm_error<S>(result_message: S) -> Self
    where
        S: Into<String>,
    {
        TxResult {
            result_status: 10,
            result_message: result_message.into(),
            ..Default::default()
        }
    }

    pub fn merge_after_sync_call(&mut self, sync_call_result: &TxResult) {
        self.result_values
            .extend_from_slice(sync_call_result.result_values.as_slice());
        self.result_logs
            .extend_from_slice(sync_call_result.result_logs.as_slice());
        if let Some(sync_result_async) = &sync_call_result.pending_calls.async_call {
            assert!(
                self.pending_calls.async_call.is_none(),
                "Multiple async calls not supported"
            );
            self.pending_calls.async_call = Some(sync_result_async.clone());
        }
    }

    pub fn assert_ok(&self) {
        assert!(
            self.result_status == 0,
            "Tx success expected, but failed. Status: {}, message: \"{}\"",
            self.result_status,
            self.result_message.as_str()
        );
    }

    pub fn assert_error(&self, expected_status: u64, expected_message: &str) {
        assert!(
            self.result_message.as_str() == expected_message,
            "Tx error message mismatch. Want status {}, message \"{}\". Have status {}, message \"{}\"",
            expected_status,
            expected_message,
            self.result_status,
            self.result_message.as_str()
        );
        assert!(
            self.result_status == expected_status,
            "Tx error status mismatch. Want status {}, message \"{}\". Have status {}, message \"{}\"",
            expected_status,
            expected_message,
            self.result_status,
            self.result_message.as_str()
        );
    }

    pub fn assert_user_error(&self, expected_message: &str) {
        self.assert_error(4, expected_message);
    }
}

impl fmt::Display for TxResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let results_hex: Vec<String> = self
            .result_values
            .iter()
            .map(|r| format!("0x{}", hex::encode(r)))
            .collect();
        write!(
            f,
            "TxResult {{\n\tresult_status: {},\n\tresult_values:{results_hex:?}\n}}",
            self.result_status
        )
    }
}

impl TxResult {
    pub fn result_values_to_string(&self) -> String {
        result_values_to_string(&self.result_values)
    }
}

pub fn result_values_to_string(values: &[Vec<u8>]) -> String {
    itertools::join(
        values.iter().map(|val| format!("0x{}", hex::encode(val))),
        ", ",
    )
}
