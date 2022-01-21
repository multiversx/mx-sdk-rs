use super::AsyncCallTxData;

#[derive(Clone, Debug)]
pub struct Promise {
    pub endpoint: AsyncCallTxData,
    pub success_callback: &'static [u8],
    pub error_callback: &'static [u8],
}
