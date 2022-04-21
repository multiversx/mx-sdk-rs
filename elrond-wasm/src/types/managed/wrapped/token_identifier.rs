use crate::{
    abi::{TypeAbi, TypeName},
    api::{Handle, ManagedTypeApi, ManagedTypeApiImpl},
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    types::{heap::BoxedBytes, ManagedBuffer, ManagedType},
};
use elrond_codec::*;

/// Specialized type for handling token identifiers.
/// It wraps a BoxedBytes with the full ASCII name of the token.
/// EGLD is stored as an empty name.
///
/// Not yet implemented, but we might add additional restrictions when deserializing as argument.
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct TokenIdentifier<M: ManagedTypeApi> {
    buffer: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for TokenIdentifier<M> {
    #[inline]
    fn from_raw_handle(handle: Handle) -> Self {
        TokenIdentifier {
            buffer: ManagedBuffer::from_raw_handle(handle),
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.buffer.get_raw_handle()
    }

    #[doc(hidden)]
    fn transmute_from_handle_ref(handle_ref: &Handle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> TokenIdentifier<M> {
    /// This special representation is interpreted as the EGLD token.
    #[allow(clippy::needless_borrow)] // clippy is wrog here, there is no other way
    pub const EGLD_REPRESENTATION: &'static [u8; 4] = &b"EGLD";

    #[inline]
    pub fn from_esdt_bytes<B: Into<ManagedBuffer<M>>>(bytes: B) -> Self {
        TokenIdentifier {
            buffer: bytes.into(),
        }
    }

    /// New instance of the special EGLD token representation.
    #[inline]
    pub fn egld() -> Self {
        TokenIdentifier {
            buffer: ManagedBuffer::new(),
        }
    }

    #[inline]
    pub fn is_egld(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    pub fn is_esdt(&self) -> bool {
        !self.is_egld()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
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
    pub fn to_esdt_identifier(&self) -> BoxedBytes {
        self.buffer.to_boxed_bytes()
    }

    #[inline]
    pub fn as_name(&self) -> BoxedBytes {
        if self.is_egld() {
            BoxedBytes::from(&Self::EGLD_REPRESENTATION[..])
        } else {
            self.buffer.to_boxed_bytes()
        }
    }

    pub fn is_valid_esdt_identifier(&self) -> bool {
        M::managed_type_impl().validate_token_identifier(self.buffer.handle)
    }
    /// Converts `"EGLD"` to `""`.
    /// Does nothing for the other values.
    fn normalize(&mut self) {
        if self.buffer == Self::EGLD_REPRESENTATION {
            self.buffer.overwrite(&[]);
        }
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for TokenIdentifier<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        let mut token_identifier = TokenIdentifier { buffer };
        token_identifier.normalize();
        token_identifier
    }
}

impl<M: ManagedTypeApi> From<&[u8]> for TokenIdentifier<M> {
    fn from(bytes: &[u8]) -> Self {
        if bytes == Self::EGLD_REPRESENTATION {
            TokenIdentifier::egld()
        } else {
            TokenIdentifier {
                buffer: ManagedBuffer::new_from_bytes(bytes),
            }
        }
    }
}

impl<M: ManagedTypeApi> PartialEq for TokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl<M: ManagedTypeApi> Eq for TokenIdentifier<M> {}

impl<M: ManagedTypeApi> NestedEncode for TokenIdentifier<M> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        if self.is_empty() {
            (&Self::EGLD_REPRESENTATION[..]).dep_encode_or_handle_err(dest, h)
        } else {
            self.buffer.dep_encode_or_handle_err(dest, h)
        }
    }
}

impl<M: ManagedTypeApi> TopEncode for TokenIdentifier<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if self.is_empty() {
            (&Self::EGLD_REPRESENTATION[..]).top_encode_or_handle_err(output, h)
        } else {
            self.buffer.top_encode_or_handle_err(output, h)
        }
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

impl<M: ManagedTypeApi> CodecSelf for TokenIdentifier<M> {}

impl<M: ManagedTypeApi> CodecFrom<&[u8]> for TokenIdentifier<M> {}

impl<M: ManagedTypeApi> CodecFrom<Vec<u8>> for TokenIdentifier<M> {}

impl<M: ManagedTypeApi> TypeAbi for TokenIdentifier<M> {
    fn type_name() -> TypeName {
        "TokenIdentifier".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for TokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if self.is_egld() {
            f.append_bytes(Self::EGLD_REPRESENTATION);
        } else {
            f.append_managed_buffer(&self.buffer);
        }
    }
}

impl<M: ManagedTypeApi> SCDisplay for &TokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        SCDisplay::fmt(*self, f);
    }
}

const EGLD_REPRESENTATION_HEX: &[u8] = b"45474C44";

impl<M: ManagedTypeApi> SCLowerHex for TokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if self.is_egld() {
            f.append_bytes(EGLD_REPRESENTATION_HEX);
        } else {
            f.append_managed_buffer_lower_hex(&self.buffer);
        }
    }
}
