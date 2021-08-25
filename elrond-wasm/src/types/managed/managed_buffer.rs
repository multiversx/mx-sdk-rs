use crate::api::InvalidSliceError;
use crate::{
    api::{Handle, ManagedTypeApi},
    types::BoxedBytes,
};
use alloc::string::String;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
};

/// A byte buffer managed by an external API.
#[derive(Debug)]
pub struct ManagedBuffer<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    pub(crate) api: M,
}

impl<M: ManagedTypeApi> ManagedBuffer<M> {
    #[inline]
    pub fn new_empty(api: M) -> Self {
        ManagedBuffer {
            handle: api.mb_new_empty(),
            api,
        }
    }

    #[inline]
    pub fn new_from_bytes(api: M, bytes: &[u8]) -> Self {
        ManagedBuffer {
            handle: api.mb_new_from_bytes(bytes),
            api,
        }
    }

    #[inline]
    pub(crate) fn new_from_raw_handle(api: M, handle: Handle) -> Self {
        ManagedBuffer { handle, api }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.api.mb_len(self.handle)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
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

impl<M: ManagedTypeApi> PartialEq for ManagedBuffer<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.api.mb_eq(self.handle, other.handle)
    }
}

impl<M: ManagedTypeApi> Eq for ManagedBuffer<M> {}

impl<M: ManagedTypeApi> TryStaticCast for ManagedBuffer<M> {}

impl<M: ManagedTypeApi> TopEncode for ManagedBuffer<M> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_specialized(self, |else_output| {
            else_output.set_slice_u8(self.to_boxed_bytes().as_slice());
        });
        Ok(())
    }
}

impl<M: ManagedTypeApi> NestedEncode for ManagedBuffer<M> {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        if dest.push_specialized(self) {
            Ok(())
        } else {
            self.to_boxed_bytes().dep_encode(dest)
        }
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        if !dest.push_specialized(self) {
            self.to_boxed_bytes().dep_encode_or_exit(dest, c, exit);
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for ManagedBuffer<M> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        input.into_specialized(|_| Err(DecodeError::UNSUPPORTED_OPERATION))
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
