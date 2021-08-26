use alloc::boxed::Box;
use elrond_codec::{
    DecodeError, NestedDecode, NestedDecodeInput, OwnedBytesNestedDecodeInput, TryStaticCast,
};

use crate::{
    api::ManagedTypeApi,
    types::{BoxedBytes, ManagedBuffer},
};

pub struct ManagedBytesNestedDecodeInput<M: ManagedTypeApi> {
    bytes_input: OwnedBytesNestedDecodeInput,
    api: M,
}

impl<M: ManagedTypeApi> ManagedBytesNestedDecodeInput<M> {
    pub fn new(bytes: Box<[u8]>, api: M) -> Self {
        ManagedBytesNestedDecodeInput {
            bytes_input: OwnedBytesNestedDecodeInput::new(bytes),
            api,
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

    fn read_into_or_exit<ExitCtx: Clone>(
        &mut self,
        into: &mut [u8],
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) {
        self.bytes_input.read_into_or_exit(into, c, exit);
    }

    #[inline]
    fn read_specialized<T: TryStaticCast>(&mut self) -> Result<Option<T>, DecodeError> {
        if T::type_eq::<ManagedBuffer<M>>() {
            let bytes = BoxedBytes::dep_decode(self)?;
            let managed_buffer = ManagedBuffer::new_from_bytes(self.api.clone(), bytes.as_slice());
            Ok(managed_buffer.try_cast())
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn read_specialized_or_exit<T: TryStaticCast, ExitCtx: Clone>(
        &mut self,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Option<T> {
        if T::type_eq::<ManagedBuffer<M>>() {
            let bytes = BoxedBytes::dep_decode_or_exit(self, c, exit);
            let managed_buffer = ManagedBuffer::new_from_bytes(self.api.clone(), bytes.as_slice());
            managed_buffer.try_cast()
        } else {
            None
        }
    }
}
