use alloc::{boxed::Box, vec::Vec};

use crate::{DecodeError, DecodeErrorHandler, TopDecode, TopDecodeInput};

pub trait TopDecodeMultiInput: Sized {
    type ValueInput: TopDecodeInput;

    /// Check if there are more arguments that can be loaded.
    fn has_next(&self) -> bool;

    /// Retrieves an input for deserializing an argument.
    /// If the loader is out of arguments, it will crash by itself with an appropriate error,
    /// without returning.
    /// Use if the next argument is optional, use `has_next` beforehand.
    fn next_value_input<H>(&mut self, h: H) -> Result<Self::ValueInput, H::HandledErr>
    where
        H: DecodeErrorHandler;

    fn next_value<T, H>(&mut self, h: H) -> Result<T, H::HandledErr>
    where
        T: TopDecode,
        H: DecodeErrorHandler,
    {
        T::top_decode_or_handle_err(self.next_value_input(h)?, h)
    }

    /// Called after retrieving all arguments to validate that extra arguments were not provided.
    fn assert_no_more_args<H>(&self, h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if self.has_next() {
            Err(h.handle_error(DecodeError::MULTI_TOO_MANY_ARGS))
        } else {
            Ok(())
        }
    }

    /// Consumes all inputs and ignores them.
    /// After executing this, assert_no_more_args should not fail.
    fn flush_ignore<H>(&mut self, h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        while self.has_next() {
            let _ = self.next_value_input(h)?;
        }
        Ok(())
    }
}

impl TopDecodeMultiInput for Vec<Vec<u8>> {
    type ValueInput = Box<[u8]>;

    fn has_next(&self) -> bool {
        !self.is_empty()
    }

    fn next_value_input<H>(&mut self, h: H) -> Result<Self::ValueInput, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if self.has_next() {
            let first = core::mem::take(&mut self[0]);
            let tail = self.split_off(1);
            *self = tail;
            Ok(first.into_boxed_slice())
        } else {
            Err(h.handle_error(DecodeError::MULTI_TOO_FEW_ARGS))
        }
    }
}
