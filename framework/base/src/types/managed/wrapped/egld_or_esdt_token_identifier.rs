use crate::{
    abi::{TypeAbi, TypeName},
    api::{HandleConstraints, ManagedTypeApi},
    codec::*,
    derive::ManagedVecItem,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    types::{ManagedBuffer, ManagedOption, ManagedRef, ManagedType, TokenIdentifier},
};

use crate as multiversx_sc; // required by the ManagedVecItem derive

/// Specialized type for handling either EGLD or ESDT token identifiers.
///
/// Equivalent to a structure of the form
/// ```
/// # use multiversx_sc::{api::ManagedTypeApi, types::TokenIdentifier};
/// enum EgldOrEsdtTokenIdentifier<'a, M: ManagedTypeApi<'a>> {
///     Egld,
///     Esdt(TokenIdentifier<'a, M>),
/// }
/// ```
///
/// It is, however more optimized than that. Its implementation is based on `ManagedOption`.
///
/// EGLD a special, invalid token identifier handle. This way we can fit it inside a single i32 in memory.
#[repr(transparent)]
#[derive(ManagedVecItem, Clone)]
pub struct EgldOrEsdtTokenIdentifier<'a, M: ManagedTypeApi<'a>> {
    data: ManagedOption<'a, M, TokenIdentifier<'a, M>>,
}

impl<'a, M: ManagedTypeApi<'a>> EgldOrEsdtTokenIdentifier<'a, M> {
    /// This special representation is interpreted as the EGLD token.
    #[allow(clippy::needless_borrow)] // clippy is wrog here, there is no other way
    pub const EGLD_REPRESENTATION: &'static [u8; 4] = &b"EGLD";

    /// New instance of the special EGLD token representation.
    #[inline]
    pub fn egld() -> Self {
        Self {
            data: ManagedOption::none(),
        }
    }

    /// ESDT instance, containing an ESDT token identifier.
    #[inline]
    pub fn esdt<TI>(token_identifier: TI) -> Self
    where
        TokenIdentifier<'a, M>: From<TI>,
    {
        Self {
            data: ManagedOption::some(TokenIdentifier::from(token_identifier)),
        }
    }

    pub fn from_opt_raw_handle(opt_handle: Option<M::ManagedBufferHandle>) -> Self {
        match opt_handle {
            Some(handle) => Self::esdt(TokenIdentifier::from_handle(handle)),
            None => Self::egld(),
        }
    }

    pub fn parse(data: ManagedBuffer<'a, M>) -> Self {
        if data == Self::EGLD_REPRESENTATION {
            Self::egld()
        } else {
            Self::esdt(TokenIdentifier::from(data))
        }
    }

    #[inline]
    pub fn is_egld(&self) -> bool {
        self.data.is_none()
    }

    #[inline]
    pub fn is_esdt(&self) -> bool {
        self.data.is_some()
    }

    #[inline]
    pub fn into_name(self) -> ManagedBuffer<'a, M> {
        self.map_or_else(
            || ManagedBuffer::from(&Self::EGLD_REPRESENTATION[..]),
            |token_identifier| token_identifier.into_managed_buffer(),
        )
    }

    /// Checks the ESDT token identifier for validity. EGLD is considered valid, no checks needed.
    ///
    /// Will fail if it encodes an invalid ESDT token identifier.
    pub fn is_valid(&self) -> bool {
        self.map_ref_or_else(
            || true,
            |token_identifier| token_identifier.is_valid_esdt_identifier(),
        )
    }

    pub fn map_or_else<U, D, F>(self, for_egld: D, for_esdt: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(TokenIdentifier<'a, M>) -> U,
    {
        self.data.map_or_else(for_egld, for_esdt)
    }

    pub fn map_ref_or_else<U, D, F>(&self, for_egld: D, for_esdt: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(&TokenIdentifier<'a, M>) -> U,
    {
        self.data.map_ref_or_else(for_egld, for_esdt)
    }

    pub fn unwrap_esdt(self) -> TokenIdentifier<'a, M> {
        self.data.unwrap_or_sc_panic("ESDT expected")
    }

    /// Representation of the object as an `Option`.
    ///
    /// Because it does not consume `self` only a reference to the ESDT token identifier can be returned.
    pub fn as_esdt_option(&self) -> Option<ManagedRef<'_, M, TokenIdentifier<'a, M>>> {
        self.data.as_option()
    }

    /// Converts `self` into an `Option`. Consumes `self` in the process.
    pub fn into_esdt_option(self) -> Option<TokenIdentifier<'a, M>> {
        self.data.into_option()
    }
}

impl<'a, M: ManagedTypeApi<'a>> PartialEq for EgldOrEsdtTokenIdentifier<'a, M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<'a, M: ManagedTypeApi<'a>> Eq for EgldOrEsdtTokenIdentifier<'a, M> {}

impl<'a, M: ManagedTypeApi<'a>> PartialEq<TokenIdentifier<'a, M>> for EgldOrEsdtTokenIdentifier<'a, M> {
    #[inline]
    fn eq(&self, other: &TokenIdentifier<'a, M>) -> bool {
        self.map_ref_or_else(
            || false,
            |self_esdt_token_identifier| self_esdt_token_identifier == other,
        )
    }
}

impl<'a, M: ManagedTypeApi<'a>> NestedEncode for EgldOrEsdtTokenIdentifier<'a, M> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        if let Some(token_identifier) = self.data.as_option() {
            token_identifier.dep_encode_or_handle_err(dest, h)
        } else {
            (&Self::EGLD_REPRESENTATION[..]).dep_encode_or_handle_err(dest, h)
        }
    }
}

impl<'a, M: ManagedTypeApi<'a>> TopEncode for EgldOrEsdtTokenIdentifier<'a, M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if let Some(token_identifier) = self.data.as_option() {
            token_identifier.top_encode_or_handle_err(output, h)
        } else {
            (&Self::EGLD_REPRESENTATION[..]).top_encode_or_handle_err(output, h)
        }
    }
}

impl<'a, M: ManagedTypeApi<'a>> NestedDecode for EgldOrEsdtTokenIdentifier<'a, M> {
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

impl<'a, M: ManagedTypeApi<'a>> TopDecode for EgldOrEsdtTokenIdentifier<'a, M> {
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

impl<'a, M> CodecFromSelf for EgldOrEsdtTokenIdentifier<'a, M> where M: ManagedTypeApi<'a> {}

impl<'a, M> CodecFrom<TokenIdentifier<'a, M>> for EgldOrEsdtTokenIdentifier<'a, M> where M: ManagedTypeApi<'a> {}
impl<'a, M> CodecFrom<&TokenIdentifier<'a, M>> for EgldOrEsdtTokenIdentifier<'a, M> where M: ManagedTypeApi<'a> {}

impl<'a, M> CodecFrom<&[u8]> for EgldOrEsdtTokenIdentifier<'a, M> where M: ManagedTypeApi<'a> {}
impl<'a, M> CodecFrom<&str> for EgldOrEsdtTokenIdentifier<'a, M> where M: ManagedTypeApi<'a> {}

impl<'a, M: ManagedTypeApi<'a>> TypeAbi for EgldOrEsdtTokenIdentifier<'a, M> {
    fn type_name() -> TypeName {
        "EgldOrEsdtTokenIdentifier".into()
    }
}

impl<'a, M: ManagedTypeApi<'a>> SCDisplay<'a> for EgldOrEsdtTokenIdentifier<'a, M> {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        if let Some(token_identifier) = self.data.as_option() {
            f.append_managed_buffer(&ManagedBuffer::from_handle(
                token_identifier.get_handle().cast_or_signal_error::<'a, M, _>(),
            ));
        } else {
            f.append_bytes(Self::EGLD_REPRESENTATION);
        }
    }
}

const EGLD_REPRESENTATION_HEX: &[u8] = b"45474C44";

impl<'a, M: ManagedTypeApi<'a>> SCLowerHex<'a> for EgldOrEsdtTokenIdentifier<'a, M> {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        if let Some(token_identifier) = self.data.as_option() {
            f.append_managed_buffer_lower_hex(&ManagedBuffer::from_handle(
                token_identifier.get_handle().cast_or_signal_error::<'a, M, _>(),
            ));
        } else {
            f.append_bytes(EGLD_REPRESENTATION_HEX);
        }
    }
}

impl<'a, M> core::fmt::Debug for EgldOrEsdtTokenIdentifier<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::alloc::string::ToString;
        if let Some(token_identifier) = self.data.as_option() {
            let token_id_str = token_identifier.to_string();
            f.debug_tuple("EgldOrEsdtTokenIdentifier::Esdt")
                .field(&token_id_str)
                .finish()
        } else {
            f.write_str("EgldOrEsdtTokenIdentifier::Egld")
        }
    }
}
