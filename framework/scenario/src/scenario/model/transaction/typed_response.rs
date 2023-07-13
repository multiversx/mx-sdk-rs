use super::{Log, TxResponse, TxResponseStatus};
use multiversx_sc::codec::{PanicErrorHandler, TopDecodeMulti};

pub struct TypedResponse<T>
where
    T: TopDecodeMulti,
{
    pub result: Result<T, TxResponseStatus>,
    pub logs: Vec<Log>,
    pub gas: u64,
    pub refund: u64,
}

impl<T> TypedResponse<T>
where
    T: TopDecodeMulti,
{
    pub fn from_raw(raw_response: &TxResponse) -> Self {
        let result: Result<T, TxResponseStatus> = if raw_response.tx_error.is_success() {
            let mut result_raw = raw_response.out.clone();
            let Ok(decoded) = T::multi_decode_or_handle_err(&mut result_raw, PanicErrorHandler);
            Ok(decoded)
        } else {
            Err(raw_response.tx_error.clone())
        };

        TypedResponse {
            result,
            logs: raw_response.logs.clone(),
            gas: raw_response.gas,
            refund: raw_response.refund,
        }
    }
}
