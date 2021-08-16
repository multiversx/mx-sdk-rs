use alloc::string::String;

use crate::api::InvalidSliceError;
// use elrond_codec::{NestedEncodeOutput, TryStaticCast};
use crate::elrond_codec::*;

use crate::{
    api::{Handle, ManagedTypeApi},
    types::BoxedBytes,
};

/// A byte buffer managed by an external API.
pub struct ManagedBuffer<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    pub(crate) api: M,
}

impl<M: ManagedTypeApi> ManagedBuffer<M> {
    pub fn new_empty(api: M) -> Self {
        ManagedBuffer {
            handle: api.mb_new_empty(),
            api: api.clone(),
        }
    }

    pub fn new_from_bytes(api: M, bytes: &[u8]) -> Self {
        ManagedBuffer {
            handle: api.mb_new_from_bytes(bytes),
            api: api.clone(),
        }
    }

    pub(crate) fn new_from_raw_handle(api: M, handle: Handle) -> Self {
        ManagedBuffer { handle, api }
    }

    pub fn len(&self) -> usize {
        self.api.mb_len(self.handle)
    }

    pub fn to_boxed_bytes(&self) -> BoxedBytes {
        self.api.mb_to_boxed_bytes(self.handle)
    }

    /// TODO: investigate the impact of using `Result<(), ()>` on the wasm output.
    pub fn load_slice(
        &self,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        self.api
            .mb_load_slice(self.handle, starting_position, dest_slice)
    }

    pub fn copy_slice(
        &self,
        starting_position: usize,
        slice_len: usize,
    ) -> Option<ManagedBuffer<M>> {
        let result_handle = self.api.mb_new_empty();
        let err_result =
            self.api
                .mb_copy_slice(self.handle, starting_position, slice_len, result_handle);
        if err_result.is_ok() {
            Some(ManagedBuffer::new_from_raw_handle(
                self.api.clone(),
                result_handle,
            ))
        } else {
            None
        }
    }

    pub fn overwrite(&mut self, value: &[u8]) {
        self.api.mb_overwrite(self.handle, value);
    }

    pub fn append(&mut self, other: &ManagedBuffer<M>) {
        self.api.mb_append(self.handle, other.handle);
    }

    pub fn append_bytes(&mut self, slice: &[u8]) {
        self.api.mb_append_bytes(self.handle, slice);
    }
}

impl<M: ManagedTypeApi> Clone for ManagedBuffer<M> {
    fn clone(&self) -> Self {
        // TODO: Optimize!!!
        ManagedBuffer {
            handle: self.api.mb_new_from_bytes(self.to_boxed_bytes().as_slice()),
            api: self.api.clone(),
        }
    }
}

impl<M: ManagedTypeApi> TryStaticCast for ManagedBuffer<M> {}

impl<M: ManagedTypeApi> NestedEncodeOutput for ManagedBuffer<M> {
    fn write(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes);
    }

    fn push_specialized<T: TryStaticCast>(&mut self, value: &T) -> bool {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            let mb_len = managed_buffer.len() as u32;
            self.append_bytes(&mb_len.to_be_bytes()[..]);
            self.append(managed_buffer);
            true
        } else {
            false
        }
    }
}

impl<M: ManagedTypeApi> TopEncode for ManagedBuffer<M> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        if !output.set_specialized(self) {
            output.set_slice_u8(self.to_boxed_bytes().as_slice());
        }
        Ok(())
    }
}

impl<M: ManagedTypeApi> NestedEncode for ManagedBuffer<M> {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        if !dest.push_specialized(self) {
            dest.write(self.to_boxed_bytes().as_slice());
        }
        Ok(())
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        _c: ExitCtx,
        _exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        if !dest.push_specialized(self) {
            dest.write(self.to_boxed_bytes().as_slice());
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for ManagedBuffer<M> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        if let Some(managed_buffer) = input.into_specialized::<ManagedBuffer<M>>() {
            Ok(managed_buffer)
        } else {
            Err(DecodeError::UNSUPPORTED_OPERATION)
        }
    }
}

impl<M: ManagedTypeApi> NestedDecode for ManagedBuffer<M> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        if let Some(managed_buffer) = input.read_specialized::<ManagedBuffer<M>>()? {
            Ok(managed_buffer)
        } else {
            Err(DecodeError::UNSUPPORTED_OPERATION)
        }
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        if let Some(managed_buffer) =
            input.read_specialized_or_exit::<ManagedBuffer<M>, ExitCtx>(c.clone(), exit)
        {
            managed_buffer
        } else {
            exit(c, DecodeError::UNSUPPORTED_OPERATION)
        }
    }
}

impl<M: ManagedTypeApi> crate::abi::TypeAbi for ManagedBuffer<M> {
    fn type_name() -> String {
        "bytes".into()
    }
}
