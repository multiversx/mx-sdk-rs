use crate::{
    contract_base::SendRawWrapper,
    types::{BigUint, ManagedAddress, ManagedVec, TxFrom, TxToSpecified},
};

use super::{
    AnnotatedEgldPayment, Egld, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment,
    TxPaymentEgldOnly,
};

impl<Env> TxPayment<Env> for ()
where
    Env: TxEnv,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        true
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        Egld(BigUint::zero()).perform_transfer_execute(env, to, gas_limit, fc);
    }

    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        _from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| f(to_addr, &BigUint::zero(), &fc))
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData::default()
    }
}

impl<Env> TxPaymentEgldOnly<Env> for () where Env: TxEnv {}
