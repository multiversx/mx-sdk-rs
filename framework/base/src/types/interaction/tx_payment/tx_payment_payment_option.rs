use crate::{
    contract_base::TransferExecuteFailed,
    types::{BigUint, ManagedAddress, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

/// TxPayment should work for any Option,
/// where for Some(payment) it behaves like payment,
/// and for None it behaves like no payment.
impl<Env, P> TxPayment<Env> for Option<P>
where
    Env: TxEnv,
    P: TxPayment<Env>,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.is_none()
    }

    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        if let Some(payment) = self {
            payment.perform_transfer_execute_fallible(env, to, gas_limit, fc)
        } else {
            ().perform_transfer_execute_fallible(env, to, gas_limit, fc)
        }
    }

    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        if let Some(payment) = self {
            payment.perform_transfer_execute_legacy(env, to, gas_limit, fc)
        } else {
            ().perform_transfer_execute_legacy(env, to, gas_limit, fc)
        }
    }

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
        if let Some(payment) = self {
            payment.with_normalized(env, from, to, fc, f)
        } else {
            ().with_normalized(env, from, to, fc, f)
        }
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        if let Some(payment) = self {
            payment.into_full_payment_data(env)
        } else {
            ().into_full_payment_data(env)
        }
    }
}
