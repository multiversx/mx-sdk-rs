use core::marker::PhantomData;

use alloc::{boxed::Box, vec::Vec};

use crate::{
    api::{ErrorApi, ErrorApiImpl, ManagedTypeApi},
    err_msg,
    types::BoxedBytes,
    DynArgInput,
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

impl<A> DynArgInput for BytesArgLoader<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    type ItemInput = Box<[u8]>;

    type ManagedTypeErrorApi = A;

    #[inline]
    fn has_next(&self) -> bool {
        self.next_index < self.bytes_vec.len()
    }

    fn next_arg_input(&mut self) -> Box<[u8]> {
        if !self.has_next() {
            A::error_api_impl().signal_error(err_msg::ARG_WRONG_NUMBER.as_bytes());
        }

        // consume from the vector, get owned bytes
        // no clone
        // no vector resize
        let boxed_bytes =
            core::mem::replace(&mut self.bytes_vec[self.next_index], BoxedBytes::empty());
        self.next_index += 1;
        boxed_bytes.into_box()
    }
}
