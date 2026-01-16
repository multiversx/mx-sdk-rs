use crate::{
    contract_base::TransferExecuteFailed,
    types::{BigUint, EsdtTokenPayment, ManagedAddress, ManagedVec, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for EsdtTokenPayment<Env::Api>
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.amount == 0u32
    }

    #[inline]
    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        self.as_refs()
            .perform_transfer_execute_fallible(env, to, gas_limit, fc)
    }

    #[inline]
    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.as_refs()
            .perform_transfer_execute_legacy(env, to, gas_limit, fc)
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
        self.as_refs().with_normalized(env, from, to, fc, f)
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: ManagedVec::from_single_item(self.into_multi_egld_or_esdt_payment()),
        }
    }
}

impl<Env> TxPayment<Env> for &EsdtTokenPayment<Env::Api>
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.amount == 0u32
    }

    #[inline]
    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        self.as_refs()
            .perform_transfer_execute_fallible(env, to, gas_limit, fc)
    }

    #[inline]
    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.as_refs()
            .perform_transfer_execute_legacy(env, to, gas_limit, fc)
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
        self.as_refs().with_normalized(env, from, to, fc, f)
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: ManagedVec::from_single_item(self.as_egld_or_esdt_payment().clone()),
        }
    }
}
