use core::marker::PhantomData;

use crate::{
    abi::TypeName,
    api::{ErrorApiImpl, Handle, InvalidSliceError, ManagedBufferApi, ManagedTypeApi},
    formatter::{
        hex_util::encode_bytes_as_hex, FormatByteReceiver, SCDisplay, SCLowerHex, HEX_VALUE_PREFIX,
    },
    types::{heap::BoxedBytes, ManagedType},
};
use elrond_codec::{
    CodecFrom, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
    TryStaticCast,
};

/// A byte buffer managed by an external API.
#[repr(transparent)]
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

    fn transmute_from_handle_ref(handle_ref: &Handle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> ManagedBuffer<M> {
    #[inline]
    pub fn new() -> Self {
        ManagedBuffer::from_raw_handle(M::managed_type_impl().mb_new_empty())
    }

    #[inline]
    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        ManagedBuffer::from_raw_handle(M::managed_type_impl().mb_new_from_bytes(bytes))
    }

    #[inline]
    pub fn new_random(nr_bytes: usize) -> Self {
        let handle = M::managed_type_impl().mb_new_empty();
        M::managed_type_impl().mb_set_random(handle, nr_bytes);

        ManagedBuffer::from_raw_handle(handle)
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

impl<M> From<crate::types::heap::Vec<u8>> for ManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(bytes: crate::types::heap::Vec<u8>) -> Self {
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
        M::managed_type_impl().mb_len(self.handle)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> BoxedBytes {
        M::managed_type_impl().mb_to_boxed_bytes(self.handle)
    }

    /// TODO: investigate the impact of using `Result<(), ()>` on the wasm output.
    #[inline]
    pub fn load_slice(
        &self,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        M::managed_type_impl().mb_load_slice(self.handle, starting_position, dest_slice)
    }

    pub fn copy_slice(
        &self,
        starting_position: usize,
        slice_len: usize,
    ) -> Option<ManagedBuffer<M>> {
        let api = M::managed_type_impl();
        let result_handle = api.mb_new_empty();
        let err_result =
            api.mb_copy_slice(self.handle, starting_position, slice_len, result_handle);
        if err_result.is_ok() {
            Some(ManagedBuffer::from_raw_handle(result_handle))
        } else {
            None
        }
    }

    pub fn load_to_byte_array<'a, const N: usize>(&self, array: &'a mut [u8; N]) -> &'a [u8] {
        let len = self.len();
        if len > N {
            M::error_api_impl().signal_error(&b"failed to load to byte array"[..]);
        }
        let byte_slice = &mut array[..len];
        let _ = self.load_slice(0, byte_slice);
        byte_slice
    }

    #[inline]
    pub fn overwrite(&mut self, value: &[u8]) {
        M::managed_type_impl().mb_overwrite(self.handle, value);
    }

    #[cfg(feature = "ei-1-1")]
    pub fn set_slice(
        &mut self,
        starting_position: usize,
        source_slice: &[u8],
    ) -> Result<(), InvalidSliceError> {
        if let Ok(()) =
            M::managed_type_impl().mb_set_slice(self.handle, starting_position, source_slice)
        {
            Ok(())
        } else {
            Err(InvalidSliceError)
        }
    }

    /// Alternate implementation that uses copies and appends to achieve the slice replacement.
    /// Should be used until EI version 1.1 is shipped to mainnet.
    #[cfg(not(feature = "ei-1-1"))]
    #[allow(clippy::collapsible_if)]
    pub fn set_slice(
        &mut self,
        starting_position: usize,
        source_slice: &[u8],
    ) -> Result<(), InvalidSliceError> {
        let api = M::managed_type_impl();
        let self_len = self.len();
        let slice_len = source_slice.len();
        if starting_position + slice_len > self_len {
            return Err(InvalidSliceError);
        }

        // copy part after update -> temporary managed buffer
        let part_after_handle = api.mb_new_empty();
        let part_after_start = starting_position + slice_len;
        let part_after_len = self_len - part_after_start;
        if part_after_len > 0 {
            if api
                .mb_copy_slice(
                    self.handle,
                    part_after_start,
                    part_after_len,
                    part_after_handle,
                )
                .is_err()
            {
                return Err(InvalidSliceError);
            }
        }

        // trim self to length of part before update
        if api
            .mb_copy_slice(self.handle, 0, starting_position, self.handle)
            .is_err()
        {
            return Err(InvalidSliceError);
        }

        // append updated slice
        api.mb_append_bytes(self.handle, source_slice);

        // copy temporary managed buffer -> part after update (using append)
        api.mb_append(self.handle, part_after_handle);

        Ok(())
    }

    pub fn set_random(&mut self, nr_bytes: usize) {
        M::managed_type_impl().mb_set_random(self.handle, nr_bytes);
    }

    #[inline]
    pub fn append(&mut self, other: &ManagedBuffer<M>) {
        M::managed_type_impl().mb_append(self.handle, other.handle);
    }

    #[inline(always)]
    pub fn append_bytes(&mut self, slice: &[u8]) {
        M::managed_type_impl().mb_append_bytes(self.handle, slice);
    }

    /// Utility function: helps serialize lengths (or any other value of type usize) easier.
    pub fn append_u32_be(&mut self, item: u32) {
        M::managed_type_impl().mb_append_bytes(self.handle, &item.to_be_bytes()[..]);
    }

    /// Convenience method for quickly getting a top-decoded u64 from the managed buffer.
    ///
    /// TODO: remove this method once TopDecodeInput is implemented for ManagedBuffer reference.
    pub fn parse_as_u64(&self) -> Option<u64> {
        const U64_NUM_BYTES: usize = 8;
        let l = self.len();
        if l > U64_NUM_BYTES {
            return None;
        }
        let mut bytes = [0u8; U64_NUM_BYTES];
        if M::managed_type_impl()
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
        let api = M::managed_type_impl();
        let clone_handle = api.mb_new_empty();
        api.mb_append(clone_handle, self.handle);
        ManagedBuffer::from_raw_handle(clone_handle)
    }
}

impl<M: ManagedTypeApi> PartialEq for ManagedBuffer<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        M::managed_type_impl().mb_eq(self.handle, other.handle)
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
        let _ = M::managed_type_impl().mb_load_slice(self.handle, 0, &mut self_bytes[..]);
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

impl<M: ManagedTypeApi> NestedEncode for ManagedBuffer<M> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        if O::supports_specialized_type::<Self>() {
            let len_bytes = (self.len() as u32).to_be_bytes();
            dest.write(&len_bytes[..]);
            dest.push_specialized((), self, h)
        } else {
            self.to_boxed_bytes().dep_encode_or_handle_err(dest, h)
        }
    }
}

impl<M: ManagedTypeApi> TopEncode for ManagedBuffer<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if O::supports_specialized_type::<Self>() {
            output.set_specialized(self, h)
        } else {
            output.set_slice_u8(self.to_boxed_bytes().as_slice());
            Ok(())
        }
    }
}

impl<M: ManagedTypeApi> CodecFrom<&[u8]> for ManagedBuffer<M> {}
impl<M: ManagedTypeApi, const N: usize> CodecFrom<&[u8; N]> for ManagedBuffer<M> {}

macro_rules! managed_buffer_codec_from_impl_bi_di {
    ($other_ty:ty) => {
        impl<M: ManagedTypeApi> CodecFrom<$other_ty> for ManagedBuffer<M> {}
        impl<M: ManagedTypeApi> CodecFrom<&$other_ty> for ManagedBuffer<M> {}
        impl<M: ManagedTypeApi> CodecFrom<ManagedBuffer<M>> for $other_ty {}
        impl<M: ManagedTypeApi> CodecFrom<&ManagedBuffer<M>> for $other_ty {}
    };
}

managed_buffer_codec_from_impl_bi_di! {crate::types::heap::Vec<u8>}
managed_buffer_codec_from_impl_bi_di! {crate::types::heap::BoxedBytes}

impl<M: ManagedTypeApi> NestedDecode for ManagedBuffer<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        if I::supports_specialized_type::<Self>() {
            input.read_specialized((), h)
        } else {
            let boxed_bytes = BoxedBytes::dep_decode_or_handle_err(input, h)?;
            Ok(Self::new_from_bytes(boxed_bytes.as_slice()))
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for ManagedBuffer<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        if I::supports_specialized_type::<Self>() {
            input.into_specialized(h)
        } else {
            Ok(ManagedBuffer::new_from_bytes(&input.into_boxed_slice_u8()))
        }
    }
}

impl<M: ManagedTypeApi> crate::abi::TypeAbi for ManagedBuffer<M> {
    fn type_name() -> TypeName {
        "bytes".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for ManagedBuffer<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        f.append_managed_buffer(self);
    }
}

impl<M: ManagedTypeApi> SCLowerHex for ManagedBuffer<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        f.append_bytes(HEX_VALUE_PREFIX); // TODO: in Rust thr `0x` prefix appears only when writing "{:#x}", not "{:x}"
        f.append_managed_buffer_lower_hex(self);
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for ManagedBuffer<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ManagedBuffer")
            .field("handle", &self.handle)
            .field(
                "hex-value",
                &encode_bytes_as_hex(self.to_boxed_bytes().as_slice()),
            )
            .finish()
    }
}
