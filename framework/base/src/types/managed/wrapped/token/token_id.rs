use alloc::string::{String, ToString};
use multiversx_chain_core::EGLD_000000_TOKEN_IDENTIFIER;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{HandleConstraints, ManagedTypeApi, ManagedTypeApiImpl, quick_signal_error},
    codec::*,
    contract_base::BlockchainWrapper,
    err_msg,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    proxy_imports::TestTokenIdentifier,
    types::{
        EgldOrEsdtTokenIdentifier, EsdtTokenIdentifier, LEGACY_EGLD_REPRESENTATION, ManagedBuffer,
        ManagedRef, ManagedType, TokenIdentifier,
    },
};

/// Specialized type for handling token identifiers (e.g. ABCDEF-123456).
#[repr(transparent)]
#[derive(Clone)]
pub struct TokenId<M: ManagedTypeApi> {
    pub(crate) buffer: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for TokenId<M> {
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    unsafe fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        unsafe {
            TokenId {
                buffer: ManagedBuffer::from_handle(handle),
            }
        }
    }

    fn get_handle(&self) -> M::ManagedBufferHandle {
        self.buffer.get_handle()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        unsafe { self.buffer.forget_into_handle() }
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::ManagedBufferHandle) -> &mut Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> TokenId<M> {
    /// Creates a new TokenId from a ManagedBuffer.
    ///
    /// Automatically handles backwards compatibility by converting:
    /// - Empty buffers to the native token (EGLD-000000)
    /// - Legacy "EGLD" representation to the native token (EGLD-000000)
    /// - All other values are stored as-is
    pub fn new(data: ManagedBuffer<M>) -> Self {
        Self::new_backwards_compatible(data)
    }

    /// Creates a new TokenId with backwards compatibility for legacy EGLD representations.
    ///
    /// This method handles the conversion of legacy EGLD representations to the current standard:
    /// - Empty buffers are converted to the native token (EGLD-000000)
    /// - The legacy "EGLD" representation is converted to "EGLD-000000"
    /// - All other token identifiers are stored without modification
    ///
    /// This ensures consistency across the codebase when dealing with the native token.
    pub fn new_backwards_compatible(data: ManagedBuffer<M>) -> Self {
        const LEGACY_EGLD_REPRESENTATION_LEN: usize = LEGACY_EGLD_REPRESENTATION.len();
        match data.len() {
            0 => Self::native(),
            LEGACY_EGLD_REPRESENTATION_LEN if data == LEGACY_EGLD_REPRESENTATION => Self::native(),
            _ => unsafe { Self::new_unchecked(data) },
        }
    }

    /// Creates a TokenId without any conversion or validation.
    ///
    /// ## Safety
    ///
    /// It does not convert the legacy EGLD representation ("EGLD" -> "EGLD-000000"). Only use if you are certain you are not in this scenario.
    pub unsafe fn new_unchecked(data: ManagedBuffer<M>) -> Self {
        Self { buffer: data }
    }

    /// Creates a TokenId representing the native token on the chain.
    ///
    /// Returns a TokenId with the value "EGLD-000000", which is the current standard
    /// representation for the native EGLD token.
    ///
    /// Future developments might make this configurable for custom chains.
    pub fn native() -> Self {
        unsafe { Self::new_unchecked(ManagedBuffer::from(EGLD_000000_TOKEN_IDENTIFIER)) }
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
        EgldOrEsdtTokenIdentifier { token_id: self }
    }

    pub fn as_legacy(&self) -> &EgldOrEsdtTokenIdentifier<M> {
        // safe because of #[repr(transparent)]
        unsafe { core::mem::transmute(self) }
    }

    /// Converts to a specialized ESDT token identifier.
    ///
    /// ## Safety
    ///
    /// Leads to inconsistencies if the token is EGLD.
    pub unsafe fn as_esdt_unchecked(&self) -> &EsdtTokenIdentifier<M> {
        // safe because of #[repr(transparent)]
        unsafe { core::mem::transmute(self) }
    }

    /// Converts to a specialized ESDT token identifier.
    ///
    /// ## Safety
    ///
    /// Leads to inconsistencies if the token is EGLD.
    pub unsafe fn into_esdt_unchecked(self) -> EsdtTokenIdentifier<M> {
        EsdtTokenIdentifier { token_id: self }
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> crate::types::heap::BoxedBytes {
        self.buffer.to_boxed_bytes()
    }

    /// Checks if a token is the native one on the chain. Currently only returns true for `EGLD-000000`.
    pub fn is_native(&self) -> bool {
        BlockchainWrapper::<M>::new().is_native_token(self)
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

impl<M: ManagedTypeApi> AsRef<TokenId<M>> for TokenId<M> {
    fn as_ref(&self) -> &TokenId<M> {
        self
    }
}

impl<M: ManagedTypeApi> AsRef<TokenId<M>> for EgldOrEsdtTokenIdentifier<M> {
    fn as_ref(&self) -> &TokenId<M> {
        self.as_token_id()
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for TokenId<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        Self::new_backwards_compatible(buffer)
    }
}

impl<M: ManagedTypeApi> From<EgldOrEsdtTokenIdentifier<M>> for TokenId<M> {
    #[inline]
    fn from(token_id: EgldOrEsdtTokenIdentifier<M>) -> Self {
        // EgldOrEsdtTokenIdentifier is also kept in memory as with the same representation as TokenId
        // EGLD legacy conversion is performed at deserialization, creation and conversion
        token_id.token_id
    }
}

impl<M: ManagedTypeApi> From<&[u8]> for TokenId<M> {
    fn from(bytes: &[u8]) -> Self {
        Self::new_backwards_compatible(ManagedBuffer::new_from_bytes(bytes))
    }
}

impl<M: ManagedTypeApi, const N: usize> From<&[u8; N]> for TokenId<M> {
    fn from(bytes: &[u8; N]) -> Self {
        Self::new_backwards_compatible(ManagedBuffer::new_from_bytes(bytes))
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
        "TokenId".into()
    }

    fn type_name_rust() -> TypeName {
        "TokenId<$API>".into()
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

impl<M: ManagedTypeApi> core::fmt::Display for TokenId<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = self.buffer.to_boxed_bytes();
        let s = alloc::string::String::from_utf8_lossy(bytes.as_slice());
        s.fmt(f)
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for TokenId<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TokenId").field(&self.to_string()).finish()
    }
}
