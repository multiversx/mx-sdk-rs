use alloc::vec::Vec;

use std::fmt;

use super::{TxLog, TxPanic, TxResultCalls};

#[derive(Clone, Default, Debug)]
pub struct TxResult {
    pub result_status: u64,
    pub result_message: String,
    pub result_values: Vec<Vec<u8>>,
    pub result_logs: Vec<TxLog>,
    pub result_calls: TxResultCalls,
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
            "TxResult {{\n\tresult_status: {},\n\tresult_values:{:?}\n}}",
            self.result_status, results_hex
        )
    }
}

impl TxResult {
    pub fn empty() -> TxResult {
        TxResult {
            result_status: 0,
            result_message: String::new(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
            result_calls: TxResultCalls::empty(),
        }
    }

    pub fn print(&self) {
        println!("{}", self);
    }

    pub fn from_panic_obj(panic_obj: &TxPanic) -> Self {
        TxResult {
            result_status: panic_obj.status,
            result_message: String::from_utf8(panic_obj.message.clone()).unwrap(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
            result_calls: TxResultCalls::empty(),
        }
    }

    pub fn from_panic_string(_s: &str) -> Self {
        TxResult {
            result_status: 4,
            result_message: "panic occurred".to_string(),
            // result_message: _s.to_string(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
            result_calls: TxResultCalls::empty(),
        }
    }

    pub fn from_unknown_panic() -> Self {
        Self::from_panic_string("")
    }

    pub fn from_vm_error(result_message: String) -> Self {
        TxResult {
            result_status: 10,
            result_message,
            result_values: Vec::new(),
            result_logs: Vec::new(),
            result_calls: TxResultCalls::empty(),
        }
    }

    pub fn merge_after_sync_call(&mut self, sync_call_result: &TxResult) {
        self.result_values
            .extend_from_slice(sync_call_result.result_values.as_slice());
        self.result_logs
            .extend_from_slice(sync_call_result.result_logs.as_slice());
        if let Some(sync_result_async) = &sync_call_result.result_calls.async_call {
            assert!(
                self.result_calls.async_call.is_none(),
                "Multiple async calls not supported"
            );
            self.result_calls.async_call = Some(sync_result_async.clone());
        }
    }
}
