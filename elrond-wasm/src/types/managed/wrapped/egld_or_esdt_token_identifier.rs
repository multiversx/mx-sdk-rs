use crate::{
    abi::{TypeAbi, TypeName},
    api::ManagedTypeApi,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    types::{ManagedBuffer, ManagedType},
};
use elrond_codec::*;

use super::{ManagedOption, TokenIdentifier};

/// Specialized type for handling either EGLD or ESDT token identifiers.
///
/// Equivalent to a structure of the form
/// ```
/// enum EgldOrEsdtTokenIdentifier<M: ManagedTypeApi> {
///     Egld,
///     Esdt(TokenIdentifier<M>),
/// }
/// ```
///
/// It is, however more optimized than that.
///
/// EGLD a special, invalid token identifier handle. This way we can fit it inside a single i32 in memory.
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct EgldOrEsdtTokenIdentifier<M: ManagedTypeApi> {
    data: ManagedOption<M, TokenIdentifier<M>>,
}

impl<M: ManagedTypeApi> EgldOrEsdtTokenIdentifier<M> {
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

    /// New instance of the special EGLD token representation.
    #[inline]
    pub fn esdt(token_identifier: TokenIdentifier<M>) -> Self {
        Self {
            data: ManagedOption::some(token_identifier),
        }
    }

    pub fn parse(data: ManagedBuffer<M>) -> Self {
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
    pub fn into_name(self) -> ManagedBuffer<M> {
        self.map_or_else(
            || ManagedBuffer::from(&Self::EGLD_REPRESENTATION[..]),
            |token_identifier| token_identifier.into_managed_buffer(),
        )
    }

    pub fn is_valid_esdt_identifier(&self) -> bool {
        self.map_ref_or_else(
            || true,
            |token_identifier| token_identifier.is_valid_esdt_identifier(),
        )
    }

    pub fn map_or_else<U, D, F>(self, for_egld: D, for_esdt: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(TokenIdentifier<M>) -> U,
    {
        self.data.map_or_else(for_egld, for_esdt)
    }

    pub fn map_ref_or_else<U, D, F>(&self, for_egld: D, for_esdt: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(&TokenIdentifier<M>) -> U,
    {
        self.data.map_ref_or_else(for_egld, for_esdt)
    }

    // pub fn as_esdt_token_identifier<'a>(&'a self) -> Option<&'a TokenIdentifier<M>> {
    //     self.map_ref_or_else(|| None, |token_identifier| Some(token_identifier))
    // }
}

// impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for EgldOrEsdtTokenIdentifier<M> {
//     #[inline]
//     fn from(buffer: ManagedBuffer<M>) -> Self {
//         let mut token_identifier = TokenIdentifier { buffer };
//         token_identifier.normalize();
//         token_identifier
//     }
// }

impl<M: ManagedTypeApi> PartialEq for EgldOrEsdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<M: ManagedTypeApi> Eq for EgldOrEsdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> NestedEncode for EgldOrEsdtTokenIdentifier<M> {
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

impl<M: ManagedTypeApi> TopEncode for EgldOrEsdtTokenIdentifier<M> {
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

impl<M: ManagedTypeApi> TypeAbi for EgldOrEsdtTokenIdentifier<M> {
    fn type_name() -> TypeName {
        "TokenIdentifier".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for EgldOrEsdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if let Some(token_identifier) = self.data.as_option() {
            f.append_managed_buffer(&ManagedBuffer::from_raw_handle(
                token_identifier.get_raw_handle(),
            ));
        } else {
            f.append_bytes(Self::EGLD_REPRESENTATION);
        }
    }
}

const EGLD_REPRESENTATION_HEX: &[u8] = b"45474C44";

impl<M: ManagedTypeApi> SCLowerHex for EgldOrEsdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if let Some(token_identifier) = self.data.as_option() {
            f.append_managed_buffer_lower_hex(&ManagedBuffer::from_raw_handle(
                token_identifier.get_raw_handle(),
            ));
        } else {
            f.append_bytes(EGLD_REPRESENTATION_HEX);
        }
    }
}
