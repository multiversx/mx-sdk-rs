use multiversx_sc_codec::Empty;

use crate::{
    contract_base::{SendRawWrapper, TransferExecuteFailed},
    types::{
        AnnotatedValue, BigUint, EgldOrEsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec,
        TxFrom, TxToSpecified,
    },
};

use super::{
    AnnotatedEgldPayment, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment,
    TxPaymentEgldOnly,
};

/// Indicates the EGLD payment in a transaction.
pub struct Egld<EgldValue>(pub EgldValue);

pub type EgldPayment<Api> = Egld<BigUint<Api>>;

impl<EgldValue: Clone> Clone for Egld<EgldValue> {
    fn clone(&self) -> Self {
        Egld(self.0.clone())
    }
}

impl<Env, EgldValue> TxPayment<Env> for Egld<EgldValue>
where
    Env: TxEnv,
    EgldValue: TxEgldValue<Env>,
{
    fn is_no_payment(&self, env: &Env) -> bool {
        self.0.with_value_ref(env, |egld_value| egld_value == &0u32)
    }

    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        self.0.with_value_ref(env, |egld_value| {
            if egld_value == &0u64 {
                // will crash
                ().perform_transfer_execute_fallible(env, to, gas_limit, fc)
            } else {
                // TODO: can probably be further optimized
                let mut payments = ManagedVec::new();
                payments.push(EgldOrEsdtTokenPayment::egld_payment(egld_value.clone()));
                SendRawWrapper::<Env::Api>::new().multi_egld_or_esdt_transfer_execute_fallible(
                    to,
                    &payments,
                    gas_limit,
                    &fc.function_name,
                    &fc.arg_buffer,
                )
            }
        })
    }

    fn perform_transfer_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        self.0.with_value_ref(env, |egld_value| {
            SendRawWrapper::<Env::Api>::new().direct_egld(to, egld_value, Empty);
        });
        Ok(())
    }

    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.0.with_value_ref(env, |egld_value| {
            SendRawWrapper::<Env::Api>::new().direct_egld_execute(
                to,
                egld_value,
                gas_limit,
                &fc.function_name,
                &fc.arg_buffer,
            );
        });
    }

    #[inline]
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
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| {
            self.0
                .with_value_ref(env, |egld_value| f(to_addr, egld_value, fc))
        })
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: Some(AnnotatedEgldPayment::new_egld(self.0.into_value(env))),
            multi_esdt: ManagedVec::new(),
        }
    }
}

impl<Env, EgldValue> AnnotatedValue<Env, BigUint<Env::Api>> for Egld<EgldValue>
where
    Env: TxEnv,
    EgldValue: TxEgldValue<Env>,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        self.0.annotation(env)
    }

    #[inline]
    fn to_value(&self, env: &Env) -> BigUint<Env::Api> {
        self.0.to_value(env)
    }

    #[inline]
    fn into_value(self, env: &Env) -> BigUint<Env::Api> {
        self.0.into_value(env)
    }

    #[inline]
    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        self.0.with_value_ref(env, f)
    }
}

impl<Env, EgldValue> TxPaymentEgldOnly<Env> for Egld<EgldValue>
where
    Env: TxEnv,
    EgldValue: TxEgldValue<Env>,
{
}
