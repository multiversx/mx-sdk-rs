use multiversx_sc_codec::IntoMultiValue;

use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{BigUint, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPaymentMultiValue},
};

use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

use crate as multiversx_sc; // needed by the TypeAbi generated code
use crate::derive::type_abi;

use super::{
    managed_vec_item_read_from_payload_index, managed_vec_item_save_to_payload_index,
    EsdtTokenPayment, EsdtTokenPaymentRefs, ManagedVec, ManagedVecItem,
    ManagedVecItemPayloadBuffer, ManagedVecRef,
};

#[type_abi]
#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug)]
pub struct EgldOrEsdtTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: EgldOrEsdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

/// Alias for a list of payments of EGLD or ESDT tokens.
pub type MultiEgldOrEsdtPayment<Api> = ManagedVec<Api, EgldOrEsdtTokenPayment<Api>>;

impl<M: ManagedTypeApi> EgldOrEsdtTokenPayment<M> {
    pub fn no_payment() -> Self {
        Self::egld_payment(BigUint::zero())
    }

    pub fn new(
        token_identifier: EgldOrEsdtTokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
        EgldOrEsdtTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// A payment of token EGLD-000000.
    pub fn egld_payment(amount: BigUint<M>) -> Self {
        Self::new(EgldOrEsdtTokenIdentifier::egld(), 0, amount)
    }

    /// Will convert to just ESDT or terminate execution if the token is EGLD.
    pub fn unwrap_esdt(self) -> EsdtTokenPayment<M> {
        EsdtTokenPayment::new(
            self.token_identifier.unwrap_esdt(),
            self.token_nonce,
            self.amount,
        )
    }

    /// Equivalent to a `match { Egld | Esdt }`.
    ///
    /// Context passed on from function to closures, to avoid ownership issues.
    /// More precisely, since only one of the two closures `for_egld` and `for_esdt` is called,
    /// it is ok for them to have fully owned access to anything from the environment.
    /// The compiler doesn't know that only one of them can ever be called,
    /// so if we pass context to both closures, it will complain that they are moved twice.
    pub fn map_egld_or_esdt<Context, D, F, U>(self, context: Context, for_egld: D, for_esdt: F) -> U
    where
        D: FnOnce(Context, BigUint<M>) -> U,
        F: FnOnce(Context, EsdtTokenPayment<M>) -> U,
    {
        self.token_identifier.map_or_else(
            (context, self.amount),
            |(context, amount)| for_egld(context, amount),
            |(context, amount), token_identifier| {
                for_esdt(
                    context,
                    EsdtTokenPayment::new(token_identifier, self.token_nonce, amount),
                )
            },
        )
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
        D: FnOnce(Context, &BigUint<M>) -> U,
        F: FnOnce(Context, EsdtTokenPaymentRefs<'_, M>) -> U,
    {
        self.token_identifier.map_ref_or_else(
            context,
            |context| for_egld(context, &self.amount),
            |context, token_identifier| {
                for_esdt(
                    context,
                    EsdtTokenPaymentRefs::new(token_identifier, self.token_nonce, &self.amount),
                )
            },
        )
    }

    pub fn into_tuple(self) -> (EgldOrEsdtTokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }
}

impl<M: ManagedTypeApi> From<(EgldOrEsdtTokenIdentifier<M>, u64, BigUint<M>)>
    for EgldOrEsdtTokenPayment<M>
{
    #[inline]
    fn from(value: (EgldOrEsdtTokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> From<EsdtTokenPayment<M>> for EgldOrEsdtTokenPayment<M> {
    fn from(esdt_payment: EsdtTokenPayment<M>) -> Self {
        EgldOrEsdtTokenPayment {
            token_identifier: EgldOrEsdtTokenIdentifier::esdt(esdt_payment.token_identifier),
            token_nonce: esdt_payment.token_nonce,
            amount: esdt_payment.amount,
        }
    }
}

impl<M: ManagedTypeApi> IntoMultiValue for EgldOrEsdtTokenPayment<M> {
    type MultiValue = EgldOrEsdtTokenPaymentMultiValue<M>;

    #[inline]
    fn into_multi_value(self) -> Self::MultiValue {
        self.into()
    }
}

impl<M> TypeAbiFrom<&[u8]> for EgldOrEsdtTokenPayment<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> EgldOrEsdtTokenPayment<M> {
    pub fn as_refs(&self) -> EgldOrEsdtTokenPaymentRefs<'_, M> {
        EgldOrEsdtTokenPaymentRefs::new(&self.token_identifier, self.token_nonce, &self.amount)
    }
}

/// Similar to `EgldOrEsdtTokenPayment`, but only contains references.
pub struct EgldOrEsdtTokenPaymentRefs<'a, M: ManagedTypeApi> {
    pub token_identifier: &'a EgldOrEsdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: &'a BigUint<M>,
}

impl<'a, M: ManagedTypeApi> EgldOrEsdtTokenPaymentRefs<'a, M> {
    pub fn new(
        token_identifier: &'a EgldOrEsdtTokenIdentifier<M>,
        token_nonce: u64,
        amount: &'a BigUint<M>,
    ) -> EgldOrEsdtTokenPaymentRefs<'a, M> {
        EgldOrEsdtTokenPaymentRefs {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    pub fn to_owned_payment(&self) -> EgldOrEsdtTokenPayment<M> {
        EgldOrEsdtTokenPayment {
            token_identifier: self.token_identifier.clone(),
            token_nonce: self.token_nonce,
            amount: self.amount.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.amount == &0u32
    }

    pub fn map_egld_or_esdt<Context, D, F, U>(self, context: Context, for_egld: D, for_esdt: F) -> U
    where
        D: FnOnce(Context, &BigUint<M>) -> U,
        F: FnOnce(Context, EsdtTokenPaymentRefs<M>) -> U,
    {
        self.token_identifier.map_ref_or_else(
            context,
            |context| for_egld(context, self.amount),
            |context, token_identifier| {
                for_esdt(
                    context,
                    EsdtTokenPaymentRefs::new(token_identifier, self.token_nonce, self.amount),
                )
            },
        )
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for EgldOrEsdtTokenPayment<M> {
    type PAYLOAD = ManagedVecItemPayloadBuffer<16>;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = ManagedVecRef<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            EgldOrEsdtTokenPayment {
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
