use super::AsyncCallTxData;

#[derive(Clone, Default, Debug)]
pub struct TxResultCalls {
    pub async_call: Option<AsyncCallTxData>,
    pub transfer_execute: Vec<AsyncCallTxData>,
}

impl TxResultCalls {
    pub fn empty() -> Self {
        TxResultCalls {
            async_call: None,
            transfer_execute: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.async_call.is_none() && self.transfer_execute.is_empty()
    }
}
