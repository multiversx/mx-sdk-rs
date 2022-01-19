use super::AsyncCallTxData;

#[derive(Debug, Clone)]
pub enum TxCallbackSelector {
    Success(AsyncCallTxData),
    Error(AsyncCallTxData),
}

impl TxCallbackSelector {
    pub fn get_tx_data(&self) -> &AsyncCallTxData {
        match &self {
            TxCallbackSelector::Success(callback) => callback,

            TxCallbackSelector::Error(callback) => callback,
        }
    }
}
