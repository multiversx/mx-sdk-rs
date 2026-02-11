use crate::{
    contract_base::TransferExecuteFailed,
    types::{
        BigUint, EsdtTokenIdentifier, EsdtTokenPayment, ManagedAddress, NonZeroBigUint, Payment, PaymentVec, TokenId, TxFrom, TxPaymentCompose, TxToSpecified
    },
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env, T, A> TxPayment<Env> for (T, u64, A)
where
    Env: TxEnv,
    T: Into<TokenId<Env::Api>>,
    A: Into<NonZeroBigUint<Env::Api>>,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        false
    }

    #[inline]
    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        Payment::from(self).perform_transfer_execute_fallible(env, to, gas_limit, fc)
    }

    #[inline]
    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        Payment::from(self).perform_transfer_execute_legacy(env, to, gas_limit, fc)
    }

    #[inline]
    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        Payment::from(self).with_normalized(env, from, to, fc, f)
    }

    #[inline]
    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        Payment::from(self).into_full_payment_data(env)
    }
}

impl<Env, T1, A1, T2, A2> TxPaymentCompose<Env, (T2, u64, A2)> for (T1, u64, A1)
where
    Env: TxEnv,
    T1: Into<TokenId<Env::Api>>,
    A1: Into<NonZeroBigUint<Env::Api>>,
    T2: Into<TokenId<Env::Api>>,
    A2: Into<NonZeroBigUint<Env::Api>>,
{
    type Output = PaymentVec<Env::Api>;

    fn compose(self, rhs: (T2, u64, A2)) -> Self::Output {
        let mut payments = PaymentVec::new();
        payments.push(Payment::from(self));
        payments.push(Payment::from(rhs));
        payments
    }
}

impl<Env, T, A> TxPaymentCompose<Env, (T, u64, A)> for PaymentVec<Env::Api>
where
    Env: TxEnv,
    T: Into<TokenId<Env::Api>>,
    A: Into<NonZeroBigUint<Env::Api>>,
{
    type Output = PaymentVec<Env::Api>;

    fn compose(mut self, rhs: (T, u64, A)) -> Self::Output {
        self.push(Payment::from(rhs));
        self
    }
}

/// Backwards compatibility.
impl<Env> TxPayment<Env> for (EsdtTokenIdentifier<Env::Api>, u64, BigUint<Env::Api>)
where
    Env: TxEnv,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.2 == 0u32
    }

    #[inline]
    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        EsdtTokenPayment::from(self).perform_transfer_execute_fallible(env, to, gas_limit, fc)
    }

    #[inline]
    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        EsdtTokenPayment::from(self).perform_transfer_execute_legacy(env, to, gas_limit, fc)
    }

    #[inline]
    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        EsdtTokenPayment::from(self).with_normalized(env, from, to, fc, f)
    }

    #[inline]
    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        EsdtTokenPayment::from(self).into_full_payment_data(env)
    }
}
