use crate::{
    api::ManagedTypeApi,
    types::{BigUint, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EsdtTokenPaymentRefs},
};

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
