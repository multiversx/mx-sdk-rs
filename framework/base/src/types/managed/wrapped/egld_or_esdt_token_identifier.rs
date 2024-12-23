use alloc::string::ToString;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{
        const_handles, use_raw_handle, ErrorApiImpl, HandleConstraints, ManagedBufferApiImpl,
        ManagedTypeApi,
    },
    codec::*,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    proxy_imports::TestTokenIdentifier,
    types::{ManagedBuffer, ManagedRef, ManagedType, TokenIdentifier},
};

pub const EGLD_000000_TOKEN_IDENTIFIER: &str = "EGLD-000000";

/// Specialized type for handling either EGLD or ESDT token identifiers.
///
/// Equivalent to a structure of the form
/// ```
/// # use multiversx_sc::{api::ManagedTypeApi, types::TokenIdentifier};
/// enum EgldOrEsdtTokenIdentifier<M: ManagedTypeApi> {
///     Egld,
///     Esdt(TokenIdentifier<M>),
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
    pub(crate) buffer: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for EgldOrEsdtTokenIdentifier<M> {
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    unsafe fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        EgldOrEsdtTokenIdentifier {
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

impl<M: ManagedTypeApi> EgldOrEsdtTokenIdentifier<M> {
    /// This special representation is interpreted as the EGLD token.
    pub const EGLD_REPRESENTATION: &'static [u8; 4] = b"EGLD";

    /// New instance of the special EGLD token representation.
    #[inline]
    pub fn egld() -> Self {
        EgldOrEsdtTokenIdentifier {
            buffer: ManagedBuffer::from(EGLD_000000_TOKEN_IDENTIFIER),
        }
    }

    /// ESDT instance, containing an ESDT token identifier.
    #[inline]
    pub fn esdt<TI>(token_identifier: TI) -> Self
    where
        TokenIdentifier<M>: From<TI>,
    {
        let ti_obj = TokenIdentifier::from(token_identifier);
        ti_obj.data
    }

    pub fn parse(data: ManagedBuffer<M>) -> Self {
        if data == Self::EGLD_REPRESENTATION {
            Self::egld()
        } else {
            Self { buffer: data }
        }
    }

    #[inline]
    pub fn is_egld(&self) -> bool {
        M::managed_type_impl().mb_overwrite(
            use_raw_handle(const_handles::MBUF_EGLD_000000),
            EGLD_000000_TOKEN_IDENTIFIER.as_bytes(),
        );
        M::managed_type_impl().mb_eq(
            use_raw_handle(const_handles::MBUF_EGLD_000000),
            self.buffer.handle.clone(),
        )
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

    pub fn map_or_else<Context, D, F, R>(self, context: Context, for_egld: D, for_esdt: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, TokenIdentifier<M>) -> R,
    {
        if self.is_egld() {
            for_egld(context)
        } else {
            unsafe { for_esdt(context, TokenIdentifier::esdt_unchecked(self)) }
        }
    }

    pub fn map_ref_or_else<Context, D, F, R>(&self, context: Context, for_egld: D, for_esdt: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, &TokenIdentifier<M>) -> R,
    {
        if self.is_egld() {
            for_egld(context)
        } else {
            unsafe {
                let token_identifier =
                    ManagedRef::<'_, M, TokenIdentifier<M>>::wrap_handle(self.get_handle());
                for_esdt(context, &token_identifier)
            }
        }
    }

    pub fn unwrap_esdt(self) -> TokenIdentifier<M> {
        self.map_or_else(
            (),
            |()| M::error_api_impl().signal_error(b"ESDT expected"),
            |(), token_identifier| token_identifier,
        )
    }

    /// Representation of the object as an `Option`.
    ///
    /// Because it does not consume `self` only a reference to the ESDT token identifier can be returned.
    pub fn as_esdt_option(&self) -> Option<ManagedRef<'_, M, TokenIdentifier<M>>> {
        if self.is_egld() {
            None
        } else {
            unsafe {
                Some(ManagedRef::<'_, M, TokenIdentifier<M>>::wrap_handle(
                    self.get_handle(),
                ))
            }
        }
    }

    /// Converts `self` into an `Option`. Consumes `self` in the process.
    pub fn into_esdt_option(self) -> Option<TokenIdentifier<M>> {
        self.map_or_else((), |()| None, |(), token_identifier| Some(token_identifier))
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for EgldOrEsdtTokenIdentifier<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        EgldOrEsdtTokenIdentifier { buffer }
    }
}

impl<M: ManagedTypeApi> From<&[u8]> for EgldOrEsdtTokenIdentifier<M> {
    fn from(bytes: &[u8]) -> Self {
        EgldOrEsdtTokenIdentifier {
            buffer: ManagedBuffer::new_from_bytes(bytes),
        }
    }
}

impl<M: ManagedTypeApi> PartialEq for EgldOrEsdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl<M: ManagedTypeApi> Eq for EgldOrEsdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> PartialEq<TokenIdentifier<M>> for EgldOrEsdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &TokenIdentifier<M>) -> bool {
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
        self.buffer.dep_encode_or_handle_err(dest, h)
    }
}

impl<M: ManagedTypeApi> TopEncode for EgldOrEsdtTokenIdentifier<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.buffer.top_encode_or_handle_err(output, h)
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

impl<M> TypeAbiFrom<TokenIdentifier<M>> for EgldOrEsdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&TokenIdentifier<M>> for EgldOrEsdtTokenIdentifier<M> where M: ManagedTypeApi {}
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
            let cast_handle = self.buffer.get_handle().cast_or_signal_error::<M, _>();
            let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
            f.append_managed_buffer(&wrap_cast);
        }
    }
}

const EGLD_REPRESENTATION_HEX: &[u8] = b"45474C44";

impl<M: ManagedTypeApi> SCLowerHex for EgldOrEsdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if self.is_egld() {
            f.append_bytes(EGLD_REPRESENTATION_HEX);
        } else {
            let cast_handle = self.buffer.get_handle().cast_or_signal_error::<M, _>();
            let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
            f.append_managed_buffer_lower_hex(&wrap_cast);
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
