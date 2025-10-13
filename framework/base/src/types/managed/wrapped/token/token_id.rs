use alloc::string::String;
use multiversx_chain_core::EGLD_000000_TOKEN_IDENTIFIER;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{quick_signal_error, HandleConstraints, ManagedTypeApi, ManagedTypeApiImpl},
    codec::*,
    err_msg,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    proxy_imports::TestTokenIdentifier,
    types::{
        EgldOrEsdtTokenIdentifier, EsdtTokenIdentifier, ManagedBuffer, ManagedRef, ManagedType,
        TokenIdentifier,
    },
};

/// Specialized type for handling token identifiers (e.g. ABCDEF-123456).
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TokenId<M: ManagedTypeApi> {
    pub(crate) buffer: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for TokenId<M> {
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    unsafe fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        TokenId {
            buffer: ManagedBuffer::from_handle(handle),
        }
    }

    fn get_handle(&self) -> M::ManagedBufferHandle {
        self.buffer.get_handle()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        self.buffer.forget_into_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::ManagedBufferHandle) -> &mut Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> TokenId<M> {
    pub fn new(data: ManagedBuffer<M>) -> Self {
        Self { buffer: data }
    }

    pub fn new_backwards_compatible(data: ManagedBuffer<M>) -> Self {
        if data == EgldOrEsdtTokenIdentifier::<M>::EGLD_REPRESENTATION {
            Self::from(EGLD_000000_TOKEN_IDENTIFIER.as_bytes())
        } else {
            Self { buffer: data }
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
    pub fn into_legacy(self) -> EgldOrEsdtTokenIdentifier<M> {
        EgldOrEsdtTokenIdentifier {
            token_id: self,
        }
    }

    pub fn as_legacy(&self) -> &EgldOrEsdtTokenIdentifier<M> {
        // safe because of #[repr(transparent)]
        unsafe { core::mem::transmute(self) }
    }

    pub fn as_esdt(&self) -> &EsdtTokenIdentifier<M> {
        // safe because of #[repr(transparent)]
        unsafe { core::mem::transmute(self) }
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> crate::types::heap::BoxedBytes {
        self.buffer.to_boxed_bytes()
    }

    /// Checks the ESDT token identifier for validity.
    ///
    /// Will fail if it encodes an invalid ESDT token identifier.
    pub fn is_valid(&self) -> bool {
        M::managed_type_impl().validate_token_identifier(self.buffer.handle.clone())
    }

    /// Old method name. Kept for easier transition. Use `is_valid` instead.
    pub fn is_valid_esdt_identifier(&self) -> bool {
        self.is_valid()
    }

    /// Extracts the ticker from the token identifier.
    ///
    /// E.g. for "ABCDEF-123456" it will return "ABCDEF".
    pub fn ticker(&self) -> ManagedBuffer<M> {
        let buffer = self.as_managed_buffer();
        let token_id_len = buffer.len();
        let ticker_len = M::managed_type_impl().get_token_ticker_len(token_id_len);
        buffer
            .copy_slice(0, ticker_len)
            .unwrap_or_else(|| quick_signal_error::<M>(err_msg::BAD_TOKEN_TICKER_FORMAT))
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for TokenId<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        TokenId { buffer }
    }
}

impl<M: ManagedTypeApi> From<EgldOrEsdtTokenIdentifier<M>> for TokenId<M> {
    #[inline]
    fn from(token_id: EgldOrEsdtTokenIdentifier<M>) -> Self {
        token_id.token_id
    }
}

impl<M: ManagedTypeApi> From<&[u8]> for TokenId<M> {
    fn from(bytes: &[u8]) -> Self {
        TokenId {
            buffer: ManagedBuffer::new_from_bytes(bytes),
        }
    }
}

impl<M: ManagedTypeApi, const N: usize> From<&[u8; N]> for TokenId<M> {
    fn from(bytes: &[u8; N]) -> Self {
        TokenId {
            buffer: ManagedBuffer::new_from_bytes(bytes),
        }
    }
}

impl<M: ManagedTypeApi> From<&str> for TokenId<M> {
    fn from(s: &str) -> Self {
        TokenId::from(s.as_bytes())
    }
}

impl<M: ManagedTypeApi> From<&String> for TokenId<M> {
    fn from(s: &String) -> Self {
        TokenId::from(s.as_bytes())
    }
}

impl<M: ManagedTypeApi> PartialEq for TokenId<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl<M: ManagedTypeApi> Eq for TokenId<M> {}

impl<M: ManagedTypeApi> NestedEncode for TokenId<M> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.buffer.dep_encode_or_handle_err(dest, h)
    }
}

impl<M: ManagedTypeApi> TopEncode for TokenId<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.buffer.top_encode_or_handle_err(output, h)
    }
}

impl<M: ManagedTypeApi> NestedDecode for TokenId<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::new_backwards_compatible(
            ManagedBuffer::dep_decode_or_handle_err(input, h)?,
        ))
    }
}

impl<M: ManagedTypeApi> TopDecode for TokenId<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::new_backwards_compatible(
            ManagedBuffer::top_decode_or_handle_err(input, h)?,
        ))
    }
}

impl<M> TypeAbiFrom<TokenIdentifier<M>> for TokenId<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&TokenIdentifier<M>> for TokenId<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&[u8]> for TokenId<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&str> for TokenId<M> where M: ManagedTypeApi {}

impl<M> TypeAbiFrom<TestTokenIdentifier<'_>> for TokenId<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&TestTokenIdentifier<'_>> for TokenId<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for TokenId<M> {}
impl<M: ManagedTypeApi> TypeAbiFrom<&Self> for TokenId<M> {}

impl<M: ManagedTypeApi> TypeAbi for TokenId<M> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "EgldOrEsdtTokenIdentifier".into()
    }

    fn type_name_rust() -> TypeName {
        "EgldOrEsdtTokenIdentifier<$API>".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for TokenId<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let cast_handle = self.buffer.get_handle().cast_or_signal_error::<M, _>();
        let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
        f.append_managed_buffer(&wrap_cast);
    }
}

impl<M: ManagedTypeApi> SCLowerHex for TokenId<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let cast_handle = self.buffer.get_handle().cast_or_signal_error::<M, _>();
        let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
        f.append_managed_buffer_lower_hex(&wrap_cast);
    }
}
