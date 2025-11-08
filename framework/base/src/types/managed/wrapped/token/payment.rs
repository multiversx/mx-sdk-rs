use generic_array::typenum::U16;
use multiversx_sc_codec::IntoMultiValue;

use crate::{
    api::ManagedTypeApi,
    types::{
        managed_vec_item_read_from_payload_index, managed_vec_item_save_to_payload_index, BigUint,
        Egld, EsdtTokenPayment, EsdtTokenPaymentRefs, EsdtTokenType, ManagedVecItem,
        ManagedVecItemPayloadBuffer, ManagedVecRef, PaymentMultiValue, TokenId,
    },
};

use crate as multiversx_sc; // needed by the codec and TypeAbi generated code
use crate::{
    codec::{
        self,
        derive::{NestedEncode, TopEncode},
        NestedDecode, TopDecode,
    },
    derive::type_abi,
};

use super::{EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment};

#[type_abi]
#[derive(TopEncode, NestedEncode, Clone, PartialEq, Eq, Debug)]
pub struct Payment<M: ManagedTypeApi> {
    pub token_identifier: TokenId<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> Payment<M> {
    #[inline]
    pub fn new(token_identifier: TokenId<M>, token_nonce: u64, amount: BigUint<M>) -> Self {
        Payment {
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
    pub fn into_tuple(self) -> (TokenId<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }

    /// Zero-cost conversion that loosens the EGLD restriction.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn as_egld_or_esdt_payment(&self) -> &EgldOrEsdtTokenPayment<M> {
        unsafe { core::mem::transmute(self) }
    }

    /// Conversion that loosens the EGLD restriction.
    pub fn into_multi_egld_or_esdt_payment(self) -> EgldOrEsdtTokenPayment<M> {
        EgldOrEsdtTokenPayment {
            token_identifier: EgldOrEsdtTokenIdentifier::esdt(self.token_identifier),
            token_nonce: self.token_nonce,
            amount: self.amount,
        }
    }

    /// Same as `map_egld_or_esdt`, but only takes a reference,
    /// and consequently, the closures also only get references.
    pub fn map_ref_egld_or_esdt<Context, D, F, U>(
        &self,
        context: Context,
        for_egld: D,
        for_esdt: F,
    ) -> U
    where
        D: FnOnce(Context, Egld<&BigUint<M>>) -> U,
        F: FnOnce(Context, EsdtTokenPaymentRefs<'_, M>) -> U,
    {
        if self.token_identifier.is_native() {
            for_egld(context, Egld(&self.amount))
        } else {
            for_esdt(
                context,
                EsdtTokenPaymentRefs::new(
                    unsafe { self.token_identifier.as_esdt_unchecked() },
                    self.token_nonce,
                    &self.amount,
                ),
            )
        }
    }

    pub fn map_egld_or_esdt<Context, D, F, U>(self, context: Context, for_egld: D, for_esdt: F) -> U
    where
        D: FnOnce(Context, Egld<BigUint<M>>) -> U,
        F: FnOnce(Context, EsdtTokenPayment<M>) -> U,
    {
        if self.token_identifier.is_native() {
            for_egld(context, Egld(self.amount))
        } else {
            for_esdt(
                context,
                EsdtTokenPayment::new(
                    unsafe { self.token_identifier.into_esdt_unchecked() },
                    self.token_nonce,
                    self.amount,
                ),
            )
        }
    }
}

impl<M: ManagedTypeApi> From<(TokenId<M>, u64, BigUint<M>)> for Payment<M> {
    #[inline]
    fn from(value: (TokenId<M>, u64, BigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> TopDecode for Payment<M> {
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

impl<M: ManagedTypeApi> NestedDecode for Payment<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Self::regular_dep_decode_or_handle_err(input, h)
    }
}

impl<M: ManagedTypeApi> Payment<M> {
    #[doc(hidden)]
    pub fn regular_dep_decode_or_handle_err<I, H>(
        input: &mut I,
        h: H,
    ) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Ok(Payment {
            token_identifier: TokenId::<M>::dep_decode_or_handle_err(input, h)?,
            token_nonce: <u64>::dep_decode_or_handle_err(input, h)?,
            amount: BigUint::<M>::dep_decode_or_handle_err(input, h)?,
        })
    }
}

impl<M: ManagedTypeApi> IntoMultiValue for Payment<M> {
    type MultiValue = PaymentMultiValue<M>;

    #[inline]
    fn into_multi_value(self) -> Self::MultiValue {
        self.into()
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for Payment<M> {
    type PAYLOAD = ManagedVecItemPayloadBuffer<U16>;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = ManagedVecRef<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            Payment {
                token_identifier: managed_vec_item_read_from_payload_index(payload, &mut index),
                token_nonce: managed_vec_item_read_from_payload_index(payload, &mut index),
                amount: managed_vec_item_read_from_payload_index(payload, &mut index),
            }
        }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        ManagedVecRef::new(Self::read_from_payload(payload))
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
pub struct PaymentRefs<'a, M: ManagedTypeApi> {
    pub token_identifier: &'a TokenId<M>,
    pub token_nonce: u64,
    pub amount: &'a BigUint<M>,
}

impl<M: ManagedTypeApi> Payment<M> {
    pub fn as_refs(&self) -> PaymentRefs<'_, M> {
        PaymentRefs::new(&self.token_identifier, self.token_nonce, &self.amount)
    }
}

impl<'a, M: ManagedTypeApi> PaymentRefs<'a, M> {
    #[inline]
    pub fn new(token_identifier: &'a TokenId<M>, token_nonce: u64, amount: &'a BigUint<M>) -> Self {
        PaymentRefs {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// Will clone the referenced values.
    pub fn to_owned_payment(&self) -> Payment<M> {
        Payment {
            token_identifier: self.token_identifier.clone(),
            token_nonce: self.token_nonce,
            amount: self.amount.clone(),
        }
    }
}
