use core::marker::PhantomData;

use alloc::boxed::Box;
use elrond_codec::{DecodeError, NestedDecodeInput, OwnedBytesNestedDecodeInput};

use crate::api::ManagedTypeApi;

/// TODO: remove
pub struct ManagedBytesNestedDecodeInput<M: ManagedTypeApi> {
    bytes_input: OwnedBytesNestedDecodeInput,
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ManagedBytesNestedDecodeInput<M> {
    pub fn new(bytes: Box<[u8]>) -> Self {
        ManagedBytesNestedDecodeInput {
            bytes_input: OwnedBytesNestedDecodeInput::new(bytes),
            _phantom: PhantomData,
        }
    }
}

impl<M: ManagedTypeApi> NestedDecodeInput for ManagedBytesNestedDecodeInput<M> {
    fn remaining_len(&self) -> usize {
        self.bytes_input.remaining_len()
    }

    fn read_into(&mut self, into: &mut [u8]) -> Result<(), DecodeError> {
        self.bytes_input.read_into(into)
    }
}
