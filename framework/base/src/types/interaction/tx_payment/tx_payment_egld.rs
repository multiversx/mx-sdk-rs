use crate::{
    contract_base::SendRawWrapper,
    types::{BigUint, ManagedAddress, ManagedVec},
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
    fn is_no_payment(&self) -> bool {
        self.0.with_egld_value(|egld_value| egld_value == &0u32)
    }

    fn perform_transfer_execute(
        self,
        _env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.0.with_egld_value(|egld_value| {
            let _ = SendRawWrapper::<Env::Api>::new().direct_egld_execute(
                to,
                egld_value,
                gas_limit,
                &fc.function_name,
                &fc.arg_buffer,
            );
        })
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: Some(AnnotatedEgldPayment::new_egld(self.0.into_value(env))),
            multi_esdt: ManagedVec::new(),
        }
    }
}

impl<Env, EgldValue> TxPaymentEgldOnly<Env> for Egld<EgldValue>
where
    Env: TxEnv,
    EgldValue: TxEgldValue<Env>,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        self.0.with_egld_value(f)
    }

    fn into_egld_payment(self, env: &Env) -> BigUint<Env::Api> {
        self.0.into_value(env)
    }
}
