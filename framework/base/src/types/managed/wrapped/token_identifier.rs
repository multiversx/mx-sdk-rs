use crate::{
    abi::{TypeAbi, TypeName},
    api::{ErrorApi, ErrorApiImpl, HandleConstraints, ManagedTypeApi, ManagedTypeApiImpl},
    codec::*,
    err_msg,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    types::{ManagedBuffer, ManagedType},
};

use super::EgldOrEsdtTokenIdentifier;

/// Specialized type for handling token identifiers.
/// It wraps a BoxedBytes with the full ASCII name of the token.
/// EGLD is stored as an empty name.
///
/// Not yet implemented, but we might add additional restrictions when deserializing as argument.
#[repr(transparent)]
#[derive(Clone)]
pub struct TokenIdentifier<'a, M: ErrorApi + ManagedTypeApi<'a>> {
    buffer: ManagedBuffer<'a, M>,
}

impl<'a, M: ManagedTypeApi<'a>> ManagedType<'a, M> for TokenIdentifier<'a, M> {
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        TokenIdentifier {
            buffer: ManagedBuffer::from_handle(handle),
        }
    }

    unsafe fn get_handle(&self) -> M::ManagedBufferHandle {
        self.buffer.get_handle()
    }

    fn take_handle(self) -> Self::OwnHandle {
        self.buffer.take_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<'a, M: ManagedTypeApi<'a>> TokenIdentifier<'a, M> {
    #[inline]
    pub fn from_esdt_bytes<B: Into<ManagedBuffer<'a, M>>>(bytes: B) -> Self {
        TokenIdentifier {
            buffer: bytes.into(),
        }
    }

    #[inline]
    pub fn into_managed_buffer(self) -> ManagedBuffer<'a, M> {
        self.buffer
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<'a, M> {
        &self.buffer
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> crate::types::heap::BoxedBytes {
        self.buffer.to_boxed_bytes()
    }

    pub fn is_valid_esdt_identifier(&self) -> bool {
        M::managed_type_impl().validate_token_identifier(self.buffer.handle.clone())
    }

    pub fn ticker(&self) -> ManagedBuffer<'a, M> {
        let token_id_len = self.buffer.len();
        let ticker_len = M::managed_type_impl().get_token_ticker_len(token_id_len);
        self.buffer
            .copy_slice(0, ticker_len)
            .unwrap_or_else(|| M::error_api_impl().signal_error(err_msg::BAD_TOKEN_TICKER_FORMAT))
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<ManagedBuffer<'a, M>> for TokenIdentifier<'a, M> {
    #[inline]
    fn from(buffer: ManagedBuffer<'a, M>) -> Self {
        TokenIdentifier { buffer }
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<&[u8]> for TokenIdentifier<'a, M> {
    fn from(bytes: &[u8]) -> Self {
        TokenIdentifier {
            buffer: ManagedBuffer::new_from_bytes(bytes),
        }
    }
}

impl<'a, M: ManagedTypeApi<'a>> From<&str> for TokenIdentifier<'a, M> {
    fn from(s: &str) -> Self {
        TokenIdentifier::from(s.as_bytes())
    }
}

impl<'a, M: ManagedTypeApi<'a>> PartialEq for TokenIdentifier<'a, M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl<'a, M: ManagedTypeApi<'a>> Eq for TokenIdentifier<'a, M> {}

impl<'a, M: ManagedTypeApi<'a>> PartialEq<EgldOrEsdtTokenIdentifier<'a, M>> for TokenIdentifier<'a, M> {
    #[inline]
    fn eq(&self, other: &EgldOrEsdtTokenIdentifier<'a, M>) -> bool {
        other.map_ref_or_else(
            || false,
            |esdt_token_identifier| esdt_token_identifier == self,
        )
    }
}

impl<'a, M: ManagedTypeApi<'a>> NestedEncode for TokenIdentifier<'a, M> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.buffer.dep_encode_or_handle_err(dest, h)
    }
}

impl<'a, M: ManagedTypeApi<'a>> TopEncode for TokenIdentifier<'a, M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.buffer.top_encode_or_handle_err(output, h)
    }
}

impl<'a, M: ManagedTypeApi<'a>> NestedDecode for TokenIdentifier<'a, M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(TokenIdentifier::from(
            ManagedBuffer::dep_decode_or_handle_err(input, h)?,
        ))
    }
}

impl<'a, M: ManagedTypeApi<'a>> TopDecode for TokenIdentifier<'a, M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(TokenIdentifier::from(
            ManagedBuffer::top_decode_or_handle_err(input, h)?,
        ))
    }
}

impl<'a, M> CodecFromSelf for TokenIdentifier<'a, M> where M: ManagedTypeApi<'a> {}

impl<'a, M> CodecFrom<&[u8]> for TokenIdentifier<'a, M> where M: ManagedTypeApi<'a> {}

impl<'a, M> CodecFrom<Vec<u8>> for TokenIdentifier<'a, M> where M: ManagedTypeApi<'a> {}

impl<'a, M: ManagedTypeApi<'a>> TypeAbi for TokenIdentifier<'a, M> {
    fn type_name() -> TypeName {
        "TokenIdentifier".into()
    }
}

impl<'a, M: ManagedTypeApi<'a>> SCDisplay<'a> for TokenIdentifier<'a, M> {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        f.append_managed_buffer(&ManagedBuffer::from_handle(
            self.buffer.get_handle().cast_or_signal_error::<'a, M, _>(),
        ));
    }
}

impl<'a, M: ManagedTypeApi<'a>> SCLowerHex<'a> for TokenIdentifier<'a, M> {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        f.append_managed_buffer_lower_hex(&ManagedBuffer::from_handle(
            self.buffer.get_handle().cast_or_signal_error::<'a, M, _>(),
        ));
    }
}

impl<'a, M: ManagedTypeApi<'a>> core::fmt::Display for TokenIdentifier<'a, M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = self.buffer.to_boxed_bytes();
        let s = alloc::string::String::from_utf8_lossy(bytes.as_slice());
        s.fmt(f)
    }
}

impl<'a, M: ManagedTypeApi<'a>> core::fmt::Debug for TokenIdentifier<'a, M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::alloc::string::ToString;
        f.debug_tuple("TokenIdentifier")
            .field(&self.to_string())
            .finish()
    }
}
