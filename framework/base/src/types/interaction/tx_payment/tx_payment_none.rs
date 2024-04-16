use crate::types::{BigUint, ManagedAddress, ManagedRef, TxFrom, TxToSpecified};

use super::{Egld, FullPaymentData, FunctionCall, TxEnv, TxPayment, TxPaymentEgldOnly};

impl<Env> TxPayment<Env> for ()
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        true
    }

    #[inline]
    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        Egld(BigUint::zero_ref()).perform_transfer_execute(env, to, gas_limit, fc);
    }

    #[inline]
    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        _from: From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(
            ManagedRef<'_, Env::Api, ManagedAddress<Env::Api>>,
            ManagedRef<'_, Env::Api, BigUint<Env::Api>>,
            &FunctionCall<Env::Api>,
        ) -> R
    {
        to.with_value_ref(env, |to_addr| f(to_addr.into(), BigUint::zero_ref(), &fc))
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData::default()
    }
}

impl<Env> TxPaymentEgldOnly<Env> for () where Env: TxEnv {}
