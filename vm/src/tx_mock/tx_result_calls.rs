use super::{AsyncCallTxData, Promise};

#[derive(Clone, Default, Debug)]
pub struct TxResultCalls {
    pub async_call: Option<AsyncCallTxData>,
    pub promises: Vec<Promise>,
}

impl TxResultCalls {
    pub fn empty() -> Self {
        TxResultCalls {
            async_call: None,
            promises: Vec::new(),
        }
    }

    pub fn no_calls(&self) -> bool {
        self.async_call.is_none() && self.promises.is_empty()
    }
}
