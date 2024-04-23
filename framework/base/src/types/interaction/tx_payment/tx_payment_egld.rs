use crate::{
    contract_base::SendRawWrapper,
    types::{
        AnnotatedValue, BigUint, ManagedAddress, ManagedBuffer, ManagedVec, TxFrom, TxToSpecified,
    },
};

use super::{
    AnnotatedEgldPayment, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment,
    TxPaymentEgldOnly,
};

/// Indicates the EGLD payment in a transaction.
pub struct Egld<EgldValue>(pub EgldValue);

pub type EgldPayment<Api> = Egld<BigUint<Api>>;

impl<Env, EgldValue> TxPayment<Env> for Egld<EgldValue>
where
    Env: TxEnv,
    EgldValue: TxEgldValue<Env>,
{
    fn is_no_payment(&self, env: &Env) -> bool {
        self.0.with_value_ref(env, |egld_value| egld_value == &0u32)
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.0.with_value_ref(env, |egld_value| {
            let _ = SendRawWrapper::<Env::Api>::new().direct_egld_execute(
                to,
                egld_value,
                gas_limit,
                &fc.function_name,
                &fc.arg_buffer,
            );
        })
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
        to.with_address_ref(env, |to_addr| {
            self.0
                .with_value_ref(env, |egld_value| f(to_addr, egld_value, &fc))
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

    fn to_value(&self, env: &Env) -> BigUint<Env::Api> {
        self.0.to_value(env)
    }

    fn into_value(self, env: &Env) -> BigUint<Env::Api> {
        self.0.into_value(env)
    }

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
