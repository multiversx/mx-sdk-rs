use core::marker::PhantomData;

use super::ManagedType;
use crate::{
    api::{Handle, InvalidSliceError, ManagedTypeApi},
    types::BoxedBytes,
};
use alloc::string::String;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast, Vec,
};

/// A byte buffer managed by an external API.
#[derive(Debug)]
pub struct ManagedBuffer<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for ManagedBuffer<M> {
    #[inline]
    fn from_raw_handle(handle: Handle) -> Self {
        ManagedBuffer {
            handle,
            _phantom: PhantomData,
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.handle
    }
}

impl<M: ManagedTypeApi> ManagedBuffer<M> {
    #[inline]
    pub fn new() -> Self {
        ManagedBuffer::from_raw_handle(M::instance().mb_new_empty())
    }

    #[inline]
    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        ManagedBuffer::from_raw_handle(M::instance().mb_new_from_bytes(bytes))
    }
}

impl<M> From<&[u8]> for ManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(bytes: &[u8]) -> Self {
        Self::new_from_bytes(bytes)
    }
}

impl<M> From<BoxedBytes> for ManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(bytes: BoxedBytes) -> Self {
        Self::new_from_bytes(bytes.as_slice())
    }
}

/// Syntactic sugar only.
impl<M, const N: usize> From<&[u8; N]> for ManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(bytes: &[u8; N]) -> Self {
        Self::new_from_bytes(bytes)
    }
}

impl<M> From<Vec<u8>> for ManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        Self::new_from_bytes(bytes.as_slice())
    }
}

impl<M: ManagedTypeApi> Default for ManagedBuffer<M> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<M: ManagedTypeApi> ManagedBuffer<M> {
    #[inline]
    pub fn len(&self) -> usize {
        M::instance().mb_len(self.handle)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> BoxedBytes {
        M::instance().mb_to_boxed_bytes(self.handle)
    }

    /// TODO: investigate the impact of using `Result<(), ()>` on the wasm output.
    #[inline]
    pub fn load_slice(
        &self,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        M::instance().mb_load_slice(self.handle, starting_position, dest_slice)
    }

    pub fn copy_slice(
        &self,
        starting_position: usize,
        slice_len: usize,
    ) -> Option<ManagedBuffer<M>> {
        let api = M::instance();
        let result_handle = api.mb_new_empty();
        let err_result =
            api.mb_copy_slice(self.handle, starting_position, slice_len, result_handle);
        if err_result.is_ok() {
            Some(ManagedBuffer::from_raw_handle(result_handle))
        } else {
            None
        }
    }

    #[inline]
    pub fn overwrite(&mut self, value: &[u8]) {
        M::instance().mb_overwrite(self.handle, value);
    }

    #[inline]
    pub fn append(&mut self, other: &ManagedBuffer<M>) {
        M::instance().mb_append(self.handle, other.handle);
    }

    #[inline(always)]
    pub fn append_bytes(&mut self, slice: &[u8]) {
        M::instance().mb_append_bytes(self.handle, slice);
    }

    /// Utility function: helps serialize lengths (or any other value of type usize) easier.
    pub fn append_u32_be(&mut self, item: u32) {
        M::instance().mb_append_bytes(self.handle, &item.to_be_bytes()[..]);
    }

    pub fn parse_as_u64(&self) -> Option<u64> {
        const U64_NUM_BYTES: usize = 8;
        let l = self.len();
        if l > U64_NUM_BYTES {
            return None;
        }
        let mut bytes = [0u8; U64_NUM_BYTES];
        if M::instance()
            .mb_load_slice(self.handle, 0, &mut bytes[U64_NUM_BYTES - l..])
            .is_err()
        {
            None
        } else {
            Some(u64::from_be_bytes(bytes))
        }
    }
}

impl<M: ManagedTypeApi> Clone for ManagedBuffer<M> {
    fn clone(&self) -> Self {
        let api = M::instance();
        let clone_handle = api.mb_new_empty();
        api.mb_append(clone_handle, self.handle);
        ManagedBuffer::from_raw_handle(clone_handle)
    }
}

impl<M: ManagedTypeApi> PartialEq for ManagedBuffer<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        M::instance().mb_eq(self.handle, other.handle)
    }
}

impl<M: ManagedTypeApi> Eq for ManagedBuffer<M> {}

impl<M: ManagedTypeApi, const N: usize> PartialEq<&[u8; N]> for ManagedBuffer<M> {
    #[allow(clippy::op_ref)] // clippy is wrong here, it is not needless
    fn eq(&self, other: &&[u8; N]) -> bool {
        if self.len() != N {
            return false;
        }
        let mut self_bytes = [0u8; N];
        let _ = M::instance().mb_load_slice(self.handle, 0, &mut self_bytes[..]);
        &self_bytes[..] == &other[..]
    }
}

impl<M: ManagedTypeApi> PartialEq<[u8]> for ManagedBuffer<M> {
    fn eq(&self, other: &[u8]) -> bool {
        // TODO: push this to the api and optimize by using a temporary handle
        let other_mb = ManagedBuffer::new_from_bytes(other);
        self == &other_mb
    }
}

impl<M: ManagedTypeApi> TryStaticCast for ManagedBuffer<M> {}

impl<M: ManagedTypeApi> TopEncode for ManagedBuffer<M> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_specialized(self, |else_output| {
            else_output.set_slice_u8(self.to_boxed_bytes().as_slice());
            Ok(())
        })
    }
}

impl<M: ManagedTypeApi> NestedEncode for ManagedBuffer<M> {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        dest.push_specialized((), self, |else_output| {
            self.to_boxed_bytes().dep_encode(else_output)
        })
    }
}

impl<M: ManagedTypeApi> TopDecode for ManagedBuffer<M> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        input.into_specialized(|_| Err(DecodeError::UNSUPPORTED_OPERATION))
    }
}

impl<M: ManagedTypeApi> NestedDecode for ManagedBuffer<M> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        input.read_specialized((), |_| Err(DecodeError::UNSUPPORTED_OPERATION))
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        input.read_specialized_or_exit((), c, exit, |_, c| {
            exit(c, DecodeError::UNSUPPORTED_OPERATION)
        })
    }
}

impl<M: ManagedTypeApi> crate::abi::TypeAbi for ManagedBuffer<M> {
    fn type_name() -> String {
        "bytes".into()
    }
}
