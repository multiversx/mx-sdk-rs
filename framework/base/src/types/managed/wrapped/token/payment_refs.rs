use crate::{
    api::ManagedTypeApi,
    types::{BigUint, Egld, EsdtTokenPaymentRefs, ManagedRef, NonZeroBigUint, Payment, TokenId},
};

/// The version of `Payment` that contains references instead of owned fields.
#[derive(Debug)]
pub struct PaymentRefs<'a, M: ManagedTypeApi> {
    pub token_identifier: ManagedRef<'a, M, TokenId<M>>,
    pub token_nonce: u64,
    pub amount: ManagedRef<'a, M, NonZeroBigUint<M>>,
}

impl<'a, M: ManagedTypeApi> PaymentRefs<'a, M> {
    #[inline]
    pub fn new(
        token_identifier: &'a TokenId<M>,
        token_nonce: u64,
        amount: &'a NonZeroBigUint<M>,
    ) -> Self {
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

    /// Mostly used for communication with the VM.
    pub fn map_egld_or_esdt<Context, D, F, U>(
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
            for_egld(context, Egld(self.amount.as_big_uint()))
        } else {
            for_esdt(
                context,
                EsdtTokenPaymentRefs::new(
                    unsafe { self.token_identifier.as_esdt_unchecked() },
                    self.token_nonce,
                    self.amount.as_big_uint(),
                ),
            )
        }
    }
}
