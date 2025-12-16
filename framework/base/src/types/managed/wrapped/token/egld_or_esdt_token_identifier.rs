use alloc::string::{String, ToString};
use multiversx_chain_core::EGLD_000000_TOKEN_IDENTIFIER;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{ErrorApiImpl, ManagedTypeApi},
    codec::*,
    err_msg,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    proxy_imports::TestTokenIdentifier,
    types::{EsdtTokenIdentifier, ManagedBuffer, ManagedRef, ManagedType, TokenId},
};

/// Specialized type for handling either EGLD or ESDT token identifiers.
///
/// Equivalent to a structure of the form
/// ```
/// # use multiversx_sc::{api::ManagedTypeApi, types::EsdtTokenIdentifier};
/// enum EgldOrEsdtTokenIdentifier<M: ManagedTypeApi> {
///     Egld,
///     Esdt(EsdtTokenIdentifier<M>),
/// }
/// ```
///
/// It is, however more optimized than that. Its implementation is based on `ManagedOption`.
///
/// EGLD is indicated by a special, invalid token identifier handle.
/// This way we can fit it inside a single i32 in memory.
#[repr(transparent)]
#[derive(Clone)]
pub struct EgldOrEsdtTokenIdentifier<M: ManagedTypeApi> {
    pub(crate) token_id: TokenId<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for EgldOrEsdtTokenIdentifier<M> {
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    unsafe fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        EgldOrEsdtTokenIdentifier {
            token_id: TokenId::from_handle(handle),
        }
    }

    fn get_handle(&self) -> M::ManagedBufferHandle {
        self.token_id.get_handle()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        self.token_id.forget_into_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::ManagedBufferHandle) -> &mut Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> EgldOrEsdtTokenIdentifier<M> {
    /// This special representation is interpreted as the EGLD token.
    pub const EGLD_REPRESENTATION: &'static [u8; 4] = b"EGLD";

    /// New instance of the special EGLD token representation.
    #[inline]
    pub fn egld() -> Self {
        EgldOrEsdtTokenIdentifier {
            token_id: TokenId::from(EGLD_000000_TOKEN_IDENTIFIER),
        }
    }

    /// ESDT instance, containing an ESDT token identifier.
    #[inline]
    pub fn esdt<TI>(token_identifier: TI) -> Self
    where
        EsdtTokenIdentifier<M>: From<TI>,
    {
        let ti_obj = EsdtTokenIdentifier::from(token_identifier);
        ti_obj.token_id.into()
    }

    pub fn parse(data: ManagedBuffer<M>) -> Self {
        if data == Self::EGLD_REPRESENTATION {
            Self::egld()
        } else {
            Self {
                token_id: data.into(),
            }
        }
    }

    #[inline]
    pub fn is_egld(&self) -> bool {
        self.token_id.is_native()
    }

    #[inline]
    pub fn is_esdt(&self) -> bool {
        !self.is_egld()
    }

    /// Returns "EGLD" or the token identifier.
    pub fn into_name(self) -> ManagedBuffer<M> {
        self.map_or_else(
            (),
            |()| ManagedBuffer::from(&Self::EGLD_REPRESENTATION[..]),
            |(), token_identifier| token_identifier.into_managed_buffer(),
        )
    }

    /// Checks the ESDT token identifier for validity. EGLD is considered valid, no checks needed.
    ///
    /// Will fail if it encodes an invalid ESDT token identifier.
    pub fn is_valid(&self) -> bool {
        self.map_ref_or_else(
            (),
            |()| true,
            |(), token_identifier| token_identifier.is_valid_esdt_identifier(),
        )
    }

    /// Converts reference to the newer, non-legacy TokenId.
    pub fn as_token_id(&self) -> &TokenId<M> {
        // safe because of #[repr(transparent)]
        unsafe { core::mem::transmute(self) }
    }

    #[inline]
    pub fn into_managed_buffer(self) -> ManagedBuffer<M> {
        self.token_id.buffer
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<M> {
        &self.token_id.buffer
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> crate::types::heap::BoxedBytes {
        self.token_id.to_boxed_bytes()
    }

    pub fn map_or_else<Context, D, F, R>(self, context: Context, for_egld: D, for_esdt: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, EsdtTokenIdentifier<M>) -> R,
    {
        if self.is_egld() {
            for_egld(context)
        } else {
            unsafe { for_esdt(context, EsdtTokenIdentifier::esdt_unchecked(self)) }
        }
    }

    pub fn map_ref_or_else<Context, D, F, R>(&self, context: Context, for_egld: D, for_esdt: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, &EsdtTokenIdentifier<M>) -> R,
    {
        if self.is_egld() {
            for_egld(context)
        } else {
            unsafe {
                let token_identifier =
                    ManagedRef::<'_, M, EsdtTokenIdentifier<M>>::wrap_handle(self.get_handle());
                for_esdt(context, &token_identifier)
            }
        }
    }

    pub fn unwrap_esdt(self) -> EsdtTokenIdentifier<M> {
        self.map_or_else(
            (),
            |()| {
                M::error_api_impl().signal_error(err_msg::TOKEN_IDENTIFIER_ESDT_EXPECTED.as_bytes())
            },
            |(), token_identifier| token_identifier,
        )
    }

    /// Representation of the object as an `Option`.
    ///
    /// Because it does not consume `self` only a reference to the ESDT token identifier can be returned.
    pub fn as_esdt_option(&self) -> Option<ManagedRef<'_, M, EsdtTokenIdentifier<M>>> {
        if self.is_egld() {
            None
        } else {
            unsafe {
                Some(ManagedRef::<'_, M, EsdtTokenIdentifier<M>>::wrap_handle(
                    self.get_handle(),
                ))
            }
        }
    }

    /// Converts `self` into an `Option`. Consumes `self` in the process.
    pub fn into_esdt_option(self) -> Option<EsdtTokenIdentifier<M>> {
        self.map_or_else((), |()| None, |(), token_identifier| Some(token_identifier))
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for EgldOrEsdtTokenIdentifier<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        EgldOrEsdtTokenIdentifier {
            token_id: buffer.into(),
        }
    }
}

impl<M: ManagedTypeApi> From<TokenId<M>> for EgldOrEsdtTokenIdentifier<M> {
    #[inline]
    fn from(token_id: TokenId<M>) -> Self {
        EgldOrEsdtTokenIdentifier { token_id }
    }
}

impl<M: ManagedTypeApi> From<&[u8]> for EgldOrEsdtTokenIdentifier<M> {
    fn from(bytes: &[u8]) -> Self {
        EgldOrEsdtTokenIdentifier {
            token_id: TokenId::from(bytes),
        }
    }
}

impl<M: ManagedTypeApi> From<&str> for EgldOrEsdtTokenIdentifier<M> {
    fn from(s: &str) -> Self {
        EgldOrEsdtTokenIdentifier::from(s.as_bytes())
    }
}

impl<M: ManagedTypeApi> From<&String> for EgldOrEsdtTokenIdentifier<M> {
    fn from(s: &String) -> Self {
        EgldOrEsdtTokenIdentifier::from(s.as_bytes())
    }
}

impl<M: ManagedTypeApi> PartialEq for EgldOrEsdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.token_id == other.token_id
    }
}

impl<M: ManagedTypeApi> Eq for EgldOrEsdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> PartialEq<EsdtTokenIdentifier<M>> for EgldOrEsdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &EsdtTokenIdentifier<M>) -> bool {
        self.map_ref_or_else(
            (),
            |()| false,
            |(), self_esdt_token_identifier| self_esdt_token_identifier == other,
        )
    }
}

impl<M: ManagedTypeApi> NestedEncode for EgldOrEsdtTokenIdentifier<M> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        if self.is_egld() {
            (&Self::EGLD_REPRESENTATION[..]).dep_encode_or_handle_err(dest, h)
        } else {
            self.token_id.dep_encode_or_handle_err(dest, h)
        }
    }
}

impl<M: ManagedTypeApi> TopEncode for EgldOrEsdtTokenIdentifier<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if self.is_egld() {
            (&Self::EGLD_REPRESENTATION[..]).top_encode_or_handle_err(output, h)
        } else {
            self.token_id.top_encode_or_handle_err(output, h)
        }
    }
}

impl<M: ManagedTypeApi> NestedDecode for EgldOrEsdtTokenIdentifier<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::parse(ManagedBuffer::dep_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl<M: ManagedTypeApi> TopDecode for EgldOrEsdtTokenIdentifier<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::parse(ManagedBuffer::top_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl<M> TypeAbiFrom<EsdtTokenIdentifier<M>> for EgldOrEsdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&EsdtTokenIdentifier<M>> for EgldOrEsdtTokenIdentifier<M> where M: ManagedTypeApi
{}
impl<M> TypeAbiFrom<&[u8]> for EgldOrEsdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&str> for EgldOrEsdtTokenIdentifier<M> where M: ManagedTypeApi {}

impl<M> TypeAbiFrom<TestTokenIdentifier<'_>> for EgldOrEsdtTokenIdentifier<M> where M: ManagedTypeApi
{}
impl<M> TypeAbiFrom<&TestTokenIdentifier<'_>> for EgldOrEsdtTokenIdentifier<M> where
    M: ManagedTypeApi
{
}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for EgldOrEsdtTokenIdentifier<M> {}
impl<M: ManagedTypeApi> TypeAbiFrom<&Self> for EgldOrEsdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> TypeAbi for EgldOrEsdtTokenIdentifier<M> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "EgldOrEsdtTokenIdentifier".into()
    }

    fn type_name_rust() -> TypeName {
        "EgldOrEsdtTokenIdentifier<$API>".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for EgldOrEsdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if self.is_egld() {
            f.append_bytes(Self::EGLD_REPRESENTATION);
        } else {
            SCDisplay::fmt(&self.token_id, f)
        }
    }
}

const EGLD_REPRESENTATION_HEX: &[u8] = b"45474C44";

impl<M: ManagedTypeApi> SCLowerHex for EgldOrEsdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if self.is_egld() {
            f.append_bytes(EGLD_REPRESENTATION_HEX);
        } else {
            SCLowerHex::fmt(&self.token_id, f)
        }
    }
}

impl<M> core::fmt::Debug for EgldOrEsdtTokenIdentifier<M>
where
    M: ManagedTypeApi,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.map_ref_or_else(
            f,
            |f| f.write_str("EgldOrEsdtTokenIdentifier::Egld"),
            |f, token_identifier| {
                let token_id_str = token_identifier.to_string();
                f.debug_tuple("EgldOrEsdtTokenIdentifier::Esdt")
                    .field(&token_id_str)
                    .finish()
            },
        )
    }
}

impl<M: ManagedTypeApi> core::fmt::Display for EgldOrEsdtTokenIdentifier<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.map_ref_or_else(
            f,
            |f| core::fmt::Display::fmt("EGLD", f),
            |f, token_identifier| core::fmt::Display::fmt(token_identifier, f),
        )
    }
}
