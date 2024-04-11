use core::marker::PhantomData;

use crate::codec::{DecodeError, DecodeErrorHandler, TopDecodeMultiInput};
use alloc::{boxed::Box, vec::Vec};

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    types::heap::BoxedBytes,
};

/// Consumes a vector of `BoxedBytes` and deserializes from the vector one by one.
pub struct BytesArgLoader<A>
where
    A: ManagedTypeApi,
{
    bytes_vec: Vec<BoxedBytes>,
    next_index: usize,
    _phantom: PhantomData<A>,
}

impl<A> BytesArgLoader<A>
where
    A: ManagedTypeApi,
{
    pub fn new(bytes_vec: Vec<BoxedBytes>) -> Self {
        BytesArgLoader {
            bytes_vec,
            next_index: 0,
            _phantom: PhantomData,
        }
    }
}

impl<A> TopDecodeMultiInput for BytesArgLoader<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    type ValueInput = Box<[u8]>;

    fn has_next(&self) -> bool {
        self.next_index < self.bytes_vec.len()
    }

    fn next_value_input<H>(&mut self, h: H) -> Result<Self::ValueInput, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if self.has_next() {
            // consume from the vector, get owned bytes
            // no clone
            // no vector resize
            let boxed_bytes =
                core::mem::replace(&mut self.bytes_vec[self.next_index], BoxedBytes::empty());
            self.next_index += 1;
            Ok(boxed_bytes.into_box())
        } else {
            Err(h.handle_error(DecodeError::MULTI_TOO_FEW_ARGS))
        }
    }

    fn flush_ignore<H>(&mut self, _h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.next_index = self.bytes_vec.len();
        Ok(())
    }
}
