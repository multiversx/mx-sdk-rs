use crate::{
    api::ManagedTypeApi,
    types::{BigUint, EsdtTokenPaymentMultiValue, EsdtTokenType, ManagedVecItem, TokenIdentifier},
};

use crate as multiversx_sc; // needed by the codec and TypeAbi generated code
use crate::{
    codec::{
        self,
        derive::{NestedEncode, TopEncode},
        IntoMultiValue, NestedDecode, TopDecode,
    },
    derive::TypeAbi,
};

#[derive(TopEncode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug)]
pub struct EsdtTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> EsdtTokenPayment<M> {
    #[inline]
    pub fn new(token_identifier: TokenIdentifier<M>, token_nonce: u64, amount: BigUint<M>) -> Self {
        EsdtTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    pub fn token_type(&self) -> EsdtTokenType {
        if self.amount != 0 {
            if self.token_nonce == 0 {
                EsdtTokenType::Fungible
            } else if self.amount == 1u64 {
                EsdtTokenType::NonFungible
            } else {
                EsdtTokenType::SemiFungible
            }
        } else {
            EsdtTokenType::Invalid
        }
    }

    #[inline]
    pub fn into_tuple(self) -> (TokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }
}

impl<M: ManagedTypeApi> From<(TokenIdentifier<M>, u64, BigUint<M>)> for EsdtTokenPayment<M> {
    #[inline]
    fn from(value: (TokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> TopDecode for EsdtTokenPayment<M> {
    fn top_decode_or_handle_err<I, H>(top_input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::TopDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        let mut nested_buffer = top_input.into_nested_buffer();
        let result = Self::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        if !codec::NestedDecodeInput::is_depleted(&nested_buffer) {
            return Err(h.handle_error(codec::DecodeError::INPUT_TOO_LONG));
        }
        Ok(result)
    }
}

impl<M: ManagedTypeApi> NestedDecode for EsdtTokenPayment<M> {
    #[cfg(not(feature = "esdt-token-payment-legacy-decode"))]
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Self::regular_dep_decode_or_handle_err(input, h)
    }

    #[cfg(feature = "esdt-token-payment-legacy-decode")]
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Self::backwards_compatible_dep_decode_or_handle_err(input, h)
    }
}

impl<M: ManagedTypeApi> EsdtTokenPayment<M> {
    #[doc(hidden)]
    pub fn regular_dep_decode_or_handle_err<I, H>(
        input: &mut I,
        h: H,
    ) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Ok(EsdtTokenPayment {
            token_identifier: TokenIdentifier::<M>::dep_decode_or_handle_err(input, h)?,
            token_nonce: <u64>::dep_decode_or_handle_err(input, h)?,
            amount: BigUint::<M>::dep_decode_or_handle_err(input, h)?,
        })
    }

    /// Deserializer version that accepts bytes encoded with an older version of the code.
    /// It uses some assumptions about the possible values of the token identifier to figure it out.
    ///
    /// More specifically:
    /// - The old encoding added an extra first byte, indicating the token type.
    /// - Token identifiers cannot be empty and cannot be very long ...
    /// - ... therefore if the bytes are shifted by 1, we should get an invalid token identifier length.
    ///
    /// Even more specifically:
    /// - having the first byte > 0 can only be explained by an older encoding
    /// - having the las byte zero can only mean 2 things:
    ///     - the token identifier is empty - but this is invalid
    ///     - we are reading an older encoding and the las token identifier length byte is the 5th instead of the 4th.
    ///
    /// **Please do not call directly in contracts, use it via the `esdt-token-payment-legacy-decode` feature flag instead.**
    ///
    /// It is only publicly exposed for testing.
    pub fn backwards_compatible_dep_decode_or_handle_err<I, H>(
        input: &mut I,
        h: H,
    ) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        let mut first_four_bytes = [0u8; 4];
        input.peek_into(&mut first_four_bytes[..], h)?;
        // old encoding detection, see method description for explanation
        let old_encoding = first_four_bytes[3] == 0 || first_four_bytes[0] > 0;
        if old_encoding {
            // clear legacy token type field, 1 byte
            let _ = input.read_byte(h)?;
        }
        Self::regular_dep_decode_or_handle_err(input, h)
    }
}

fn managed_vec_item_from_slice<T>(arr: &[u8], index: &mut usize) -> T
where
    T: ManagedVecItem,
{
    ManagedVecItem::from_byte_reader(|bytes| {
        let size = T::PAYLOAD_SIZE;
        bytes.copy_from_slice(&arr[*index..*index + size]);
        *index += size;
    })
}

fn managed_vec_item_to_slice<T>(arr: &mut [u8], index: &mut usize, item: &T)
where
    T: ManagedVecItem,
{
    ManagedVecItem::to_byte_writer(item, |bytes| {
        let size = T::PAYLOAD_SIZE;
        arr[*index..*index + size].copy_from_slice(bytes);
        *index += size;
    });
}

impl<M: ManagedTypeApi> IntoMultiValue for EsdtTokenPayment<M> {
    type MultiValue = EsdtTokenPaymentMultiValue<M>;

    #[inline]
    fn into_multi_value(self) -> Self::MultiValue {
        self.into()
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for EsdtTokenPayment<M> {
    const PAYLOAD_SIZE: usize = 16;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        let mut arr: [u8; 16] = [0u8; 16];
        reader(&mut arr[..]);
        let mut index = 0;

        let token_identifier = managed_vec_item_from_slice(&arr, &mut index);
        let token_nonce = managed_vec_item_from_slice(&arr, &mut index);
        let amount = managed_vec_item_from_slice(&arr, &mut index);

        EsdtTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        let mut arr: [u8; 16] = [0u8; 16];
        let mut index = 0;

        managed_vec_item_to_slice(&mut arr, &mut index, &self.token_identifier);
        managed_vec_item_to_slice(&mut arr, &mut index, &self.token_nonce);
        managed_vec_item_to_slice(&mut arr, &mut index, &self.amount);

        writer(&arr[..])
    }
}
