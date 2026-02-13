use generic_array::typenum::U16;

use crate::{
    api::ManagedTypeApi,
    types::{
        BigUint, EsdtTokenIdentifier, EsdtTokenPaymentMultiValue, EsdtTokenType, ManagedType, ManagedVec, ManagedVecItem, ManagedVecItemPayloadBuffer, Payment, PaymentVec, Ref, managed_vec_item_read_from_payload_index, managed_vec_item_save_to_payload_index
    },
};

use crate as multiversx_sc; // needed by the codec and TypeAbi generated code
use crate::{
    codec::{
        self, IntoMultiValue, NestedDecode, TopDecode,
        derive::{NestedEncode, TopEncode},
    },
    derive::type_abi,
};

use super::{EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, MultiEgldOrEsdtPayment};

#[type_abi]
#[derive(TopEncode, NestedEncode, Clone, PartialEq, Eq, Debug)]
pub struct EsdtTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: EsdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

/// Alias for a list of payments.
///
/// Deprecated: Use [`EsdtTokenPaymentVec`] instead.
#[deprecated(
    since = "0.65.0",
    note = "Replace with either `PaymentVec` (modern implementation), or `EsdtTokenPaymentVec` (backwards-compatible, identical with the old `MultiEsdtPayment`)."
)]
pub type MultiEsdtPayment<Api> = ManagedVec<Api, EsdtTokenPayment<Api>>;

/// Alias for a list of payments.
pub type EsdtTokenPaymentVec<Api> = ManagedVec<Api, EsdtTokenPayment<Api>>;

impl<M: ManagedTypeApi> EsdtTokenPayment<M> {
    #[inline]
    pub fn new(
        token_identifier: EsdtTokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
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
    pub fn into_tuple(self) -> (EsdtTokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }

    /// Conversion that loosens the EGLD restriction.
    pub fn into_egld_or_esdt_payment(self) -> EgldOrEsdtTokenPayment<M> {
        EgldOrEsdtTokenPayment {
            token_identifier: EgldOrEsdtTokenIdentifier::esdt(self.token_identifier),
            token_nonce: self.token_nonce,
            amount: self.amount,
        }
    }

    /// Converts this `EsdtTokenPayment` into a [`Payment`], enforcing a non-zero amount.
    ///
    /// # Panics
    ///
    /// This function will panic if the amount is zero, as [`Payment`] requires a non-zero amount.
    /// Use this only when you are certain the amount is non-zero, or handle zero amounts beforehand.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use multiversx_sc::types::*;
    /// # use multiversx_sc::api::ManagedTypeApi;
    /// # fn example<M: ManagedTypeApi>() -> Payment<M>{
    /// let esdt_payment = EsdtTokenPayment::new(
    ///     EsdtTokenIdentifier::from("TOKEN-123456"),
    ///     0,
    ///     BigUint::from(100u64),
    /// );
    /// let payment = esdt_payment.into_payment();
    /// # payment
    /// # }
    /// ```
    pub fn into_payment(self) -> Payment<M> {
        Payment {
            token_identifier: self.token_identifier.token_id,
            token_nonce: self.token_nonce,
            amount: self.amount.into_non_zero_or_panic(),
        }
    }
}

impl<M: ManagedTypeApi> From<(EsdtTokenIdentifier<M>, u64, BigUint<M>)> for EsdtTokenPayment<M> {
    #[inline]
    fn from(value: (EsdtTokenIdentifier<M>, u64, BigUint<M>)) -> Self {
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
            token_identifier: EsdtTokenIdentifier::<M>::dep_decode_or_handle_err(input, h)?,
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

impl<M: ManagedTypeApi> IntoMultiValue for EsdtTokenPayment<M> {
    type MultiValue = EsdtTokenPaymentMultiValue<M>;

    #[inline]
    fn into_multi_value(self) -> Self::MultiValue {
        self.into()
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for EsdtTokenPayment<M> {
    type PAYLOAD = ManagedVecItemPayloadBuffer<U16>;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = Ref<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            EsdtTokenPayment {
                token_identifier: managed_vec_item_read_from_payload_index(payload, &mut index),
                token_nonce: managed_vec_item_read_from_payload_index(payload, &mut index),
                amount: managed_vec_item_read_from_payload_index(payload, &mut index),
            }
        }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        unsafe { Ref::new(Self::read_from_payload(payload)) }
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        let mut index = 0;

        unsafe {
            managed_vec_item_save_to_payload_index(self.token_identifier, payload, &mut index);
            managed_vec_item_save_to_payload_index(self.token_nonce, payload, &mut index);
            managed_vec_item_save_to_payload_index(self.amount, payload, &mut index);
        }
    }
}

/// The version of `EsdtTokenPayment` that contains references instead of owned fields.
pub struct EsdtTokenPaymentRefs<'a, M: ManagedTypeApi> {
    pub token_identifier: &'a EsdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: &'a BigUint<M>,
}

impl<M: ManagedTypeApi> EsdtTokenPayment<M> {
    pub fn as_refs(&self) -> EsdtTokenPaymentRefs<'_, M> {
        EsdtTokenPaymentRefs::new(&self.token_identifier, self.token_nonce, &self.amount)
    }
}

impl<'a, M: ManagedTypeApi> EsdtTokenPaymentRefs<'a, M> {
    #[inline]
    pub fn new(
        token_identifier: &'a EsdtTokenIdentifier<M>,
        token_nonce: u64,
        amount: &'a BigUint<M>,
    ) -> Self {
        EsdtTokenPaymentRefs {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// Will clone the referenced values.
    pub fn to_owned_payment(&self) -> EsdtTokenPayment<M> {
        EsdtTokenPayment {
            token_identifier: self.token_identifier.clone(),
            token_nonce: self.token_nonce,
            amount: self.amount.clone(),
        }
    }
}

impl<M: ManagedTypeApi> MultiEsdtPayment<M> {
    /// Zero-cost conversion that loosens the EGLD restriction.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn as_multi_egld_or_esdt_payment(&self) -> &MultiEgldOrEsdtPayment<M> {
        unsafe { core::mem::transmute(self) }
    }

    /// Zero-cost conversion that loosens the EGLD restriction.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn into_multi_egld_or_esdt_payment(self) -> MultiEgldOrEsdtPayment<M> {
        unsafe { MultiEgldOrEsdtPayment::from_handle(self.forget_into_handle()) }
    }

    /// Zero-cost conversion that loosens the EGLD restriction.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn into_payment_vec(self) -> PaymentVec<M> {
        // safe, because it is the same layout
        unsafe { PaymentVec::from_handle(self.forget_into_handle()) }
    }
}

impl<M: ManagedTypeApi> From<()> for MultiEsdtPayment<M> {
    #[inline]
    fn from(_value: ()) -> Self {
        MultiEsdtPayment::new()
    }
}

impl<M: ManagedTypeApi> From<EsdtTokenPayment<M>> for MultiEsdtPayment<M> {
    #[inline]
    fn from(value: EsdtTokenPayment<M>) -> Self {
        MultiEsdtPayment::from_single_item(value)
    }
}

impl<M: ManagedTypeApi> From<(EsdtTokenIdentifier<M>, u64, BigUint<M>)> for MultiEsdtPayment<M> {
    #[inline]
    fn from(value: (EsdtTokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        MultiEsdtPayment::from_single_item(value.into())
    }
}
