use super::{AsyncCallTxData, TxCallbackSelector};

#[derive(Clone, Default, Debug)]
pub struct TxResultCalls {
    pub async_call: Option<AsyncCallTxData>,
    pub transfer_execute: Vec<AsyncCallTxData>,
    pub success_callback: Option<TxCallbackSelector>,
    pub error_callback: Option<TxCallbackSelector>,
}

impl TxResultCalls {
    pub fn empty() -> Self {
        TxResultCalls {
            async_call: None,
            transfer_execute: Vec::new(),
            success_callback: None,
            error_callback: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.async_call.is_none()
            && self.transfer_execute.is_empty()
            && self.success_callback.is_none()
            && self.error_callback.is_none()
    }
}
