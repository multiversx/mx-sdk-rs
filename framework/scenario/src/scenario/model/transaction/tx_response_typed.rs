use std::marker::PhantomData;

use multiversx_sc::codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti};

use super::{TxError, TxResponse};

#[derive(Debug, Default, Clone)]
pub struct TypedTxResponse<OriginalResult> {
    pub response: TxResponse,
    _phatom: PhantomData<OriginalResult>,
}

impl<OriginalResult> TypedTxResponse<OriginalResult> {
    pub fn new(response: TxResponse) -> Self {
        Self {
            response,
            _phatom: PhantomData,
        }
    }

    pub fn result<RequestedResult>(&self) -> Result<RequestedResult, TxError>
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        self.response.handle_signal_error_event()?;
        let mut raw_result = self.response.raw_result()?;
        Ok(
            RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler)
                .unwrap(),
        )
    }
}
