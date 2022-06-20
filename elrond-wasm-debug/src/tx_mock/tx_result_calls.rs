use super::{AsyncCallTxData, Promise, TxInputESDTOrEGLD};

#[derive(Clone, Default, Debug)]
pub struct TxResultCalls {
    pub async_call: Option<AsyncCallTxData>,
    pub transfer_execute_calls: TxInputESDTOrEGLD,
    pub promises: Vec<Promise>,
}

impl TxResultCalls {
    pub fn empty() -> Self {
        TxResultCalls {
            async_call: None,
            transfer_execute_calls: TxInputESDTOrEGLD::default(),
            promises: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.async_call.is_none() && self.promises.is_empty()
    }
}
