use super::{AsyncCallTxData, TxFunctionName};

#[derive(Clone, Debug)]
pub struct Promise {
    pub call: AsyncCallTxData,
    pub success_callback: TxFunctionName,
    pub error_callback: TxFunctionName,
    pub callback_closure_data: Vec<u8>,
}
