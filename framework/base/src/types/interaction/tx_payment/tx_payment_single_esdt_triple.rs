use crate::{
    contract_base::TransferExecuteFailed,
    types::{BigUint, EsdtTokenPayment, ManagedAddress, TokenIdentifier, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for (TokenIdentifier<Env::Api>, u64, BigUint<Env::Api>)
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
