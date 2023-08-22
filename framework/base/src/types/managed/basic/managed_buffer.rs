use crate::{
    abi::TypeName,
    api::{
        use_raw_handle, ErrorApiImpl, HandleConstraints, InvalidSliceError, ManagedBufferApiImpl,
        ManagedTypeApi, StaticVarApiImpl,
    },
    codec::{
        CodecFrom, CodecFromSelf, DecodeErrorHandler, Empty, EncodeErrorHandler, NestedDecode,
        NestedDecodeInput, NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode,
        TopEncodeOutput, TryStaticCast,
    },
    formatter::{
        hex_util::encode_bytes_as_hex, FormatByteReceiver, SCBinary, SCDisplay, SCLowerHex,
    },
    types::{heap::BoxedBytes, ManagedType, StaticBufferRef},
};

/// A byte buffer managed by an external API.
#[repr(transparent)]
pub struct ManagedBuffer<M: ManagedTypeApi> {
    pub(crate) handle: M::ManagedBufferHandle,
}

impl<M: ManagedTypeApi> ManagedType<M> for ManagedBuffer<M> {
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        ManagedBuffer { handle }
    }

    fn get_handle(&self) -> M::ManagedBufferHandle {
        self.handle.clone()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> ManagedBuffer<M> {
    #[inline]
    pub fn new() -> Self {
        let new_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        // TODO: remove after VM no longer crashes with "unknown handle":
        M::managed_type_impl().mb_overwrite(new_handle.clone(), &[]);
        ManagedBuffer::from_handle(new_handle)
    }

    #[inline]
    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        let new_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mb_overwrite(new_handle.clone(), bytes);
        ManagedBuffer::from_handle(new_handle)
    }

    #[inline]
    pub fn new_random(nr_bytes: usize) -> Self {
        let new_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().mb_set_random(new_handle.clone(), nr_bytes);
        ManagedBuffer::from_handle(new_handle)
    }

    fn load_static_cache(&self) -> StaticBufferRef<M>
    where
        M: ManagedTypeApi,
    {
        StaticBufferRef::try_new_from_copy_bytes(self.len(), |dest_slice| {
            let _ = self.load_slice(0, dest_slice);
        })
        .unwrap_or_else(|| {
            M::error_api_impl().signal_error(b"static cache too small or already in use")
        })
    }

    pub fn with_buffer_contents<R, F>(&self, f: F) -> R
    where
        M: ManagedTypeApi,
        F: FnOnce(&[u8]) -> R,
    {
        let static_cache = self.load_static_cache();
        static_cache.with_buffer_contents(f)
    }

    pub fn with_buffer_contents_mut<F>(&mut self, f: F)
    where
        M: ManagedTypeApi,
        F: FnOnce(&mut [u8]) -> &[u8],
    {
        let static_cache = self.load_static_cache();
        static_cache.with_buffer_contents_mut(|buffer| {
            let result = f(buffer);
            self.overwrite(result);
        });
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

impl<M> From<&str> for ManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(s: &str) -> Self {
        Self::new_from_bytes(s.as_bytes())
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

impl<M> From<Empty> for ManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from(_: Empty) -> Self {
        Self::new()
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
        M::managed_type_impl().mb_len(self.handle.clone())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Method provided for convenience in tests, not to be used in contracts.
    pub fn to_boxed_bytes(&self) -> BoxedBytes {
        M::managed_type_impl().mb_to_boxed_bytes(self.handle.clone())
    }

    /// Method provided for convenience in tests, not to be used in contracts.
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> alloc::vec::Vec<u8> {
        self.to_boxed_bytes().into_vec()
    }

    /// TODO: investigate the impact of using `Result<(), ()>` on the wasm output.
    #[inline]
    pub fn load_slice(
        &self,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        M::managed_type_impl().mb_load_slice(self.handle.clone(), starting_position, dest_slice)
    }

    pub fn copy_slice(
        &self,
        starting_position: usize,
        slice_len: usize,
    ) -> Option<ManagedBuffer<M>> {
        let api = M::managed_type_impl();
        let result_handle = api.mb_new_empty();
        let err_result = api.mb_copy_slice(
            self.handle.clone(),
            starting_position,
            slice_len,
            result_handle.clone(),
        );
        if err_result.is_ok() {
            Some(ManagedBuffer::from_handle(result_handle))
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

    /// Loads all bytes of the managed buffer in batches, then applies given closure on each batch.
    pub fn for_each_batch<const BATCH_SIZE: usize, F: FnMut(&[u8])>(&self, mut f: F) {
        let mut buffer = [0u8; BATCH_SIZE];
        let arg_len = self.len();
        let mut current_arg_index = 0;
        while current_arg_index < arg_len {
            let bytes_remaining = arg_len - current_arg_index;
            let bytes_to_load = core::cmp::min(bytes_remaining, BATCH_SIZE);
            let loaded_slice = &mut buffer[0..bytes_to_load];
            let _ = self.load_slice(current_arg_index, loaded_slice);
            f(loaded_slice);
            current_arg_index += BATCH_SIZE;
        }
    }

    #[inline]
    pub fn overwrite(&mut self, value: &[u8]) {
        M::managed_type_impl().mb_overwrite(self.handle.clone(), value);
    }

    pub fn set_slice(
        &mut self,
        starting_position: usize,
        source_slice: &[u8],
    ) -> Result<(), InvalidSliceError> {
        if let Ok(()) = M::managed_type_impl().mb_set_slice(
            self.handle.clone(),
            starting_position,
            source_slice,
        ) {
            Ok(())
        } else {
            Err(InvalidSliceError)
        }
    }

    pub fn set_random(&mut self, nr_bytes: usize) {
        M::managed_type_impl().mb_set_random(self.handle.clone(), nr_bytes);
    }

    #[inline]
    pub fn append(&mut self, other: &ManagedBuffer<M>) {
        M::managed_type_impl().mb_append(self.handle.clone(), other.handle.clone());
    }

    #[inline(always)]
    pub fn append_bytes(&mut self, slice: &[u8]) {
        M::managed_type_impl().mb_append_bytes(self.handle.clone(), slice);
    }

    /// Utility function: helps serialize lengths (or any other value of type usize) easier.
    pub fn append_u32_be(&mut self, item: u32) {
        M::managed_type_impl().mb_append_bytes(self.handle.clone(), &item.to_be_bytes()[..]);
    }

    /// Concatenates 2 managed buffers. Consumes both arguments in the process.
    #[inline]
    #[must_use]
    pub fn concat(mut self, other: ManagedBuffer<M>) -> Self {
        self.append(&other);
        self
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
            .mb_load_slice(self.handle.clone(), 0, &mut bytes[U64_NUM_BYTES - l..])
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
        api.mb_append(clone_handle.clone(), self.handle.clone());
        ManagedBuffer::from_handle(clone_handle)
    }
}

impl<M: ManagedTypeApi> PartialEq for ManagedBuffer<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        M::managed_type_impl().mb_eq(self.handle.clone(), other.handle.clone())
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
        let _ = M::managed_type_impl().mb_load_slice(self.handle.clone(), 0, &mut self_bytes[..]);
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

impl<M> CodecFromSelf for ManagedBuffer<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<&[u8]> for ManagedBuffer<M> where M: ManagedTypeApi {}
impl<M> CodecFrom<&str> for ManagedBuffer<M> where M: ManagedTypeApi {}
impl<M, const N: usize> CodecFrom<&[u8; N]> for ManagedBuffer<M> where M: ManagedTypeApi {}

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
managed_buffer_codec_from_impl_bi_di! {crate::types::heap::String}

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
        f.append_managed_buffer(&ManagedBuffer::from_handle(
            self.get_handle().cast_or_signal_error::<M, _>(),
        ));
    }
}

impl<M: ManagedTypeApi> SCLowerHex for ManagedBuffer<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let hex_handle: M::ManagedBufferHandle =
            use_raw_handle(crate::api::const_handles::MBUF_TEMPORARY_1);
        M::managed_type_impl().mb_to_hex(self.handle.clone(), hex_handle.clone());
        f.append_managed_buffer(&ManagedBuffer::from_handle(
            hex_handle.cast_or_signal_error::<M, _>(),
        ));
    }
}

impl<M: ManagedTypeApi> SCBinary for ManagedBuffer<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        // TODO: in Rust thr `0b` prefix appears only when writing "{:#x}", not "{:x}"
        f.append_managed_buffer_binary(&ManagedBuffer::from_handle(
            self.get_handle().cast_or_signal_error::<M, _>(),
        ));
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for ManagedBuffer<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ManagedBuffer")
            .field("handle", &self.handle.clone())
            .field(
                "hex-value",
                &encode_bytes_as_hex(self.to_boxed_bytes().as_slice()),
            )
            .finish()
    }
}

#[cfg(feature = "alloc")]
impl<M: ManagedTypeApi> core::fmt::Display for ManagedBuffer<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::contract_base::ErrorHelper;

        let s = alloc::string::String::from_utf8(self.to_boxed_bytes().into_vec())
            .unwrap_or_else(|err| ErrorHelper::<M>::signal_error_with_message(err.as_bytes()));

        s.fmt(f)
    }
}
