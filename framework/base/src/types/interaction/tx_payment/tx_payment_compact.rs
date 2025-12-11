use core::ops::Deref;

use crate::{
    contract_base::TransferExecuteFailed,
    types::{BigUint, Compact, ManagedAddress, ManagedRef, PaymentVec, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for Compact<&PaymentVec<Env::Api>>
where
    Env: TxEnv,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.0.is_empty()
    }

    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        match self.0.len() {
            0 => ().perform_transfer_execute_fallible(env, to, gas_limit, fc),
            1 => self
                .0
                .get(0)
                .perform_transfer_execute_fallible(env, to, gas_limit, fc),
            _ => self
                .0
                .perform_transfer_execute_fallible(env, to, gas_limit, fc),
        }
    }

    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        match self.0.len() {
            0 => ().perform_transfer_execute_legacy(env, to, gas_limit, fc),
            1 => self
                .0
                .get(0)
                .perform_transfer_execute_legacy(env, to, gas_limit, fc),
            _ => self
                .0
                .perform_transfer_execute_legacy(env, to, gas_limit, fc),
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
        match self.0.len() {
            0 => ().with_normalized(env, from, to, fc, f),
            1 => self.0.get(0).with_normalized(env, from, to, fc, f),
            _ => self.0.with_normalized(env, from, to, fc, f),
        }
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        match self.0.len() {
            0 => ().into_full_payment_data(env),
            1 => self.0.get(0).into_full_payment_data(env),
            _ => self.0.into_full_payment_data(env),
        }
    }
}

impl<Env> TxPayment<Env> for Compact<ManagedRef<'_, Env::Api, PaymentVec<Env::Api>>>
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.0.is_empty()
    }

    #[inline]
    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        Compact(self.0.deref()).perform_transfer_execute_fallible(env, to, gas_limit, fc)
    }

    #[inline]
    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        Compact(self.0.deref()).perform_transfer_execute_legacy(env, to, gas_limit, fc)
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
        Compact(self.0.deref()).with_normalized(env, from, to, fc, f)
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        Compact(self.0.deref()).into_full_payment_data(env)
    }
}
