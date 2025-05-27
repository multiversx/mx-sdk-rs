use alloc::string::ToString;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{ErrorApi, ErrorApiImpl, HandleConstraints, ManagedTypeApi, ManagedTypeApiImpl},
    codec::*,
    err_msg,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    types::{ManagedBuffer, ManagedType},
};

use super::{EgldOrEsdtTokenIdentifier, ManagedRef};

/// Specialized type for handling token identifiers.
/// It wraps a BoxedBytes with the full ASCII name of the token.
/// EGLD is stored as an empty name.
///
/// Not yet implemented, but we might add additional restrictions when deserializing as argument.
#[repr(transparent)]
#[derive(Clone)]
pub struct TokenIdentifier<M: ErrorApi + ManagedTypeApi> {
    pub(crate) data: EgldOrEsdtTokenIdentifier<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for TokenIdentifier<M> {
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    unsafe fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        TokenIdentifier {
            data: EgldOrEsdtTokenIdentifier::from_handle(handle),
        }
    }

    fn get_handle(&self) -> M::ManagedBufferHandle {
        self.data.get_handle()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        self.data.forget_into_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::ManagedBufferHandle) -> &mut Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> TokenIdentifier<M> {
    /// Creates a new TokenIdentifier without verifying that it is not EGLD-000000.
    ///
    /// ## Safety
    ///
    /// Calling it for the EGLD token might lead to unexpected bugs.
    pub unsafe fn esdt_unchecked(data: EgldOrEsdtTokenIdentifier<M>) -> Self {
        Self { data }
    }

    pub fn try_new(data: EgldOrEsdtTokenIdentifier<M>) -> Option<Self> {
        if data.is_egld() {
            return None;
        }

        unsafe { Some(Self::esdt_unchecked(data)) }
    }

    #[inline]
    pub fn from_esdt_bytes<B: Into<ManagedBuffer<M>>>(bytes: B) -> Self {
        TokenIdentifier::from(bytes.into())
    }

    #[inline]
    pub fn into_managed_buffer(self) -> ManagedBuffer<M> {
        self.data.into_managed_buffer()
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<M> {
        self.data.as_managed_buffer()
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> crate::types::heap::BoxedBytes {
        self.data.to_boxed_bytes()
    }

    pub fn is_valid_esdt_identifier(&self) -> bool {
        M::managed_type_impl().validate_token_identifier(self.data.buffer.handle.clone())
    }

    pub fn ticker(&self) -> ManagedBuffer<M> {
        let buffer = self.as_managed_buffer();
        let token_id_len = buffer.len();
        let ticker_len = M::managed_type_impl().get_token_ticker_len(token_id_len);
        buffer.copy_slice(0, ticker_len).unwrap_or_else(|| {
            M::error_api_impl().signal_error(err_msg::BAD_TOKEN_TICKER_FORMAT.as_bytes())
        })
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for TokenIdentifier<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        EgldOrEsdtTokenIdentifier::from(buffer).unwrap_esdt()
    }
}

impl<M: ManagedTypeApi> From<&[u8]> for TokenIdentifier<M> {
    fn from(bytes: &[u8]) -> Self {
        EgldOrEsdtTokenIdentifier::from(bytes).unwrap_esdt()
    }
}

impl<M: ManagedTypeApi> From<&str> for TokenIdentifier<M> {
    fn from(s: &str) -> Self {
        TokenIdentifier::from(s.as_bytes())
    }
}

impl<M: ManagedTypeApi> From<&crate::types::heap::String> for TokenIdentifier<M> {
    fn from(s: &crate::types::heap::String) -> Self {
        TokenIdentifier::from(s.as_bytes())
    }
}

impl<M: ManagedTypeApi> PartialEq for TokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<M: ManagedTypeApi> Eq for TokenIdentifier<M> {}

impl<M: ManagedTypeApi> PartialEq<EgldOrEsdtTokenIdentifier<M>> for TokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &EgldOrEsdtTokenIdentifier<M>) -> bool {
        other.map_ref_or_else(
            (),
            |()| false,
            |(), esdt_token_identifier| esdt_token_identifier == self,
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
        self.data.dep_encode_or_handle_err(dest, h)
    }
}

impl<M: ManagedTypeApi> TopEncode for TokenIdentifier<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.data.top_encode_or_handle_err(output, h)
    }
}

impl<M: ManagedTypeApi> NestedDecode for TokenIdentifier<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let data = EgldOrEsdtTokenIdentifier::dep_decode_or_handle_err(input, h)?;
        if let Some(ti) = TokenIdentifier::try_new(data) {
            Ok(ti)
        } else {
            Err(h.handle_error(err_msg::TOKEN_IDENTIFIER_ESDT_EXPECTED.into()))
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for TokenIdentifier<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let data = EgldOrEsdtTokenIdentifier::top_decode_or_handle_err(input, h)?;
        if let Some(ti) = TokenIdentifier::try_new(data) {
            Ok(ti)
        } else {
            Err(h.handle_error(err_msg::TOKEN_IDENTIFIER_ESDT_EXPECTED.into()))
        }
    }
}

impl<M> TypeAbiFrom<&[u8]> for TokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<Vec<u8>> for TokenIdentifier<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for TokenIdentifier<M> {}
impl<M: ManagedTypeApi> TypeAbiFrom<&Self> for TokenIdentifier<M> {}

impl<M: ManagedTypeApi> TypeAbi for TokenIdentifier<M> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "TokenIdentifier".into()
    }

    fn type_name_rust() -> TypeName {
        "TokenIdentifier<$API>".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for TokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let cast_handle = self.get_handle().cast_or_signal_error::<M, _>();
        let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
        f.append_managed_buffer(&wrap_cast);
    }
}

impl<M: ManagedTypeApi> SCLowerHex for TokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let cast_handle = self.get_handle().cast_or_signal_error::<M, _>();
        let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
        f.append_managed_buffer_lower_hex(&wrap_cast);
    }
}

impl<M: ManagedTypeApi> core::fmt::Display for TokenIdentifier<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = self.to_boxed_bytes();
        let s = alloc::string::String::from_utf8_lossy(bytes.as_slice());
        s.fmt(f)
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for TokenIdentifier<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TokenIdentifier")
            .field(&self.to_string())
            .finish()
    }
}
