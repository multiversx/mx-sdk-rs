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
pub struct TokenIdentifier<M: ErrorApi + ManagedTypeApi> {
    buffer: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for TokenIdentifier<M> {
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        TokenIdentifier {
            buffer: ManagedBuffer::from_handle(handle),
        }
    }

    fn get_handle(&self) -> M::ManagedBufferHandle {
        self.buffer.get_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> TokenIdentifier<M> {
    #[inline]
    pub fn from_esdt_bytes<B: Into<ManagedBuffer<M>>>(bytes: B) -> Self {
        TokenIdentifier {
            buffer: bytes.into(),
        }
    }

    #[inline]
    pub fn into_managed_buffer(self) -> ManagedBuffer<M> {
        self.buffer
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<M> {
        &self.buffer
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> crate::types::heap::BoxedBytes {
        self.buffer.to_boxed_bytes()
    }

    pub fn is_valid_esdt_identifier(&self) -> bool {
        M::managed_type_impl().validate_token_identifier(self.buffer.handle.clone())
    }

    pub fn ticker(&self) -> ManagedBuffer<M> {
        let token_id_len = self.buffer.len();
        let ticker_len = M::managed_type_impl().get_token_ticker_len(token_id_len);
        self.buffer
            .copy_slice(0, ticker_len)
            .unwrap_or_else(|| M::error_api_impl().signal_error(err_msg::BAD_TOKEN_TICKER_FORMAT))
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for TokenIdentifier<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        TokenIdentifier { buffer }
    }
}

impl<M: ManagedTypeApi> From<&[u8]> for TokenIdentifier<M> {
    fn from(bytes: &[u8]) -> Self {
        TokenIdentifier {
            buffer: ManagedBuffer::new_from_bytes(bytes),
        }
    }
}

impl<M: ManagedTypeApi> From<&str> for TokenIdentifier<M> {
    fn from(s: &str) -> Self {
        TokenIdentifier::from(s.as_bytes())
    }
}

impl<M: ManagedTypeApi> PartialEq for TokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl<M: ManagedTypeApi> Eq for TokenIdentifier<M> {}

impl<M: ManagedTypeApi> PartialEq<EgldOrEsdtTokenIdentifier<M>> for TokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &EgldOrEsdtTokenIdentifier<M>) -> bool {
        other.map_ref_or_else(
            || false,
            |esdt_token_identifier| esdt_token_identifier == self,
        )
    }
}

impl<M: ManagedTypeApi> NestedEncode for TokenIdentifier<M> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.buffer.dep_encode_or_handle_err(dest, h)
    }
}

impl<M: ManagedTypeApi> TopEncode for TokenIdentifier<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.buffer.top_encode_or_handle_err(output, h)
    }
}

impl<M: ManagedTypeApi> NestedDecode for TokenIdentifier<M> {
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

impl<M: ManagedTypeApi> TopDecode for TokenIdentifier<M> {
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

impl<M> CodecFromSelf for TokenIdentifier<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<&[u8]> for TokenIdentifier<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<Vec<u8>> for TokenIdentifier<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> TypeAbi for TokenIdentifier<M> {
    fn type_name() -> TypeName {
        "TokenIdentifier".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for TokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        f.append_managed_buffer(&ManagedBuffer::from_handle(
            self.buffer.get_handle().cast_or_signal_error::<M, _>(),
        ));
    }
}

impl<M: ManagedTypeApi> SCLowerHex for TokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        f.append_managed_buffer_lower_hex(&ManagedBuffer::from_handle(
            self.buffer.get_handle().cast_or_signal_error::<M, _>(),
        ));
    }
}

impl<M: ManagedTypeApi> core::fmt::Display for TokenIdentifier<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = self.buffer.to_boxed_bytes();
        let s = alloc::string::String::from_utf8_lossy(bytes.as_slice());
        s.fmt(f)
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for TokenIdentifier<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::alloc::string::ToString;
        f.debug_tuple("TokenIdentifier")
            .field(&self.to_string())
            .finish()
    }
}
