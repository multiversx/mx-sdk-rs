use crate::{
    contract_base::SendRawWrapper,
    types::{BigUint, ManagedAddress, ManagedVec},
};

use super::{
    AnnotatedEgldPayment, Egld, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment,
    TxPaymentEgldOnly,
};

impl<Env> TxPayment<Env> for ()
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool {
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

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData::default()
    }
}

impl<Env> TxPaymentEgldOnly<Env> for ()
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(&BigUint::zero())
    }

    fn into_egld_payment(self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::zero()
    }
}
