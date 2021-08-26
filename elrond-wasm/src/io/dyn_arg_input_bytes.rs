use alloc::vec::Vec;

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    err_msg,
    types::{BoxedBytes, ManagedBytesTopDecodeInput},
    DynArgInput,
};

/// Consumes a vector of `BoxedBytes` and deserializes from the vector one by one.
pub struct BytesArgLoader<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    bytes_vec: Vec<BoxedBytes>,
    next_index: usize,
    api: A,
}

impl<A> BytesArgLoader<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    pub fn new(api: A, bytes_vec: Vec<BoxedBytes>) -> Self {
        BytesArgLoader {
            bytes_vec,
            next_index: 0,
            api,
        }
    }
}

impl<A> ErrorApi for BytesArgLoader<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn signal_error(&self, message: &[u8]) -> ! {
        self.api.signal_error(message)
    }
}

impl<A> DynArgInput<ManagedBytesTopDecodeInput<A>> for BytesArgLoader<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    #[inline]
    fn has_next(&self) -> bool {
        self.next_index < self.bytes_vec.len()
    }

    fn next_arg_input(&mut self) -> ManagedBytesTopDecodeInput<A> {
        if !self.has_next() {
            self.signal_error(err_msg::ARG_WRONG_NUMBER);
        }

        // consume from the vector, get owned bytes
        // no clone
        // no vector resize
        let boxed_bytes =
            core::mem::replace(&mut self.bytes_vec[self.next_index], BoxedBytes::empty());
        self.next_index += 1;
        ManagedBytesTopDecodeInput::new(self.api.clone(), boxed_bytes)
    }
}
