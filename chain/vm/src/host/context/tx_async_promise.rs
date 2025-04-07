use super::{AsyncCallTxData, TxFunctionName};

#[derive(Clone, Debug)]
pub struct Promise {
    pub call: AsyncCallTxData,
    pub success_callback: TxFunctionName,
    pub error_callback: TxFunctionName,
    pub callback_closure_data: Vec<u8>,
}

impl Promise {
    pub fn has_callback(&self) -> bool {
        !self.success_callback.is_empty() && !self.error_callback.is_empty()
    }
}
