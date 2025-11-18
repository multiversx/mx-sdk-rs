use crate::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedRef, Payment, TokenId},
};

/// The version of `Payment` that contains references instead of owned fields.
#[derive(Debug)]
pub struct PaymentRefs<'a, M: ManagedTypeApi> {
    pub token_identifier: ManagedRef<'a, M, TokenId<M>>,
    pub token_nonce: u64,
    pub amount: ManagedRef<'a, M, BigUint<M>>,
}

impl<'a, M: ManagedTypeApi> PaymentRefs<'a, M> {
    #[inline]
    pub fn new(token_identifier: &'a TokenId<M>, token_nonce: u64, amount: &'a BigUint<M>) -> Self {
        PaymentRefs {
            token_identifier: ManagedRef::new(token_identifier),
            token_nonce,
            amount: ManagedRef::new(amount),
        }
    }

    /// Will clone the referenced values.
    pub fn to_owned(&self) -> Payment<M> {
        Payment {
            token_identifier: self.token_identifier.clone(),
            token_nonce: self.token_nonce,
            amount: self.amount.clone(),
        }
    }
}
