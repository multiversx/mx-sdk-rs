use alloc::vec::Vec;

use crate::{EncodeError, EncodeErrorHandler, TopEncode, TryStaticCast};

pub trait TopEncodeMultiOutput {
    fn push_single_value<T, H>(&mut self, arg: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TopEncode,
        H: EncodeErrorHandler;

    /// This is temporary, will remove after we get rid of SCResult for good.
    fn push_multi_specialized<T, H>(&mut self, _arg: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TryStaticCast,
        H: EncodeErrorHandler,
    {
        Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
    }
}

impl TopEncodeMultiOutput for Vec<Vec<u8>> {
    fn push_single_value<T, H>(&mut self, arg: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TopEncode,
        H: EncodeErrorHandler,
    {
        let mut result = Vec::new();
        arg.top_encode_or_handle_err(&mut result, h)?;
        self.push(result);
        Ok(())
    }
}
