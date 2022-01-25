use super::AsyncCallTxData;

#[derive(Clone, Default, Debug)]
pub struct TxResultCalls {
    pub async_call: Option<AsyncCallTxData>,
}

impl TxResultCalls {
    pub fn empty() -> Self {
        TxResultCalls { async_call: None }
    }

    pub fn is_empty(&self) -> bool {
        self.async_call.is_none()
    }
}
