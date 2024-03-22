mod tx_payment_egld;
mod tx_payment_egld_value;
mod tx_payment_multi_esdt;
mod tx_payment_none;
mod tx_payment_normalize;
mod tx_payment_other;
mod tx_payment_single_esdt;

pub use tx_payment_egld::{Egld, EgldPayment};
pub use tx_payment_egld_value::TxEgldValue;
pub use tx_payment_normalize::TxPaymentNormalize;

use crate::{
    api::ManagedTypeApi,
    contract_base::SendRawWrapper,
    types::{
        BigUint, EgldOrEsdtTokenPayment, EgldOrMultiEsdtPayment, EsdtTokenPayment, ManagedAddress,
        ManagedBuffer, MultiEsdtPayment,
    },
};

use super::{FunctionCall, TxEnv};

/// Describes a payment that is part of a transaction.
pub trait TxPayment<Env>
where
    Env: TxEnv,
{
    fn is_no_payment(&self) -> bool;

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    );

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api>;
}

/// Marks a payment object that only contains EGLD or nothing at all.
pub trait TxPaymentEgldOnly<Env>: TxPayment<Env>
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R;

    fn into_egld_payment(self, env: &Env) -> BigUint<Env::Api>;
}

#[derive(Clone)]
pub struct AnnotatedEgldPayment<Api>
where
    Api: ManagedTypeApi,
{
    pub value: BigUint<Api>,
    pub annotation: ManagedBuffer<Api>,
}

impl<Api> AnnotatedEgldPayment<Api>
where
    Api: ManagedTypeApi,
{
    pub fn new_egld(value: BigUint<Api>) -> Self {
        let annotation = value.to_display();
        AnnotatedEgldPayment { value, annotation }
    }
}

#[derive(Clone)]
pub struct FullPaymentData<Api>
where
    Api: ManagedTypeApi,
{
    pub egld: Option<AnnotatedEgldPayment<Api>>,
    pub multi_esdt: MultiEsdtPayment<Api>,
}

impl<Api> Default for FullPaymentData<Api>
where
    Api: ManagedTypeApi,
{
    fn default() -> Self {
        Self {
            egld: None,
            multi_esdt: Default::default(),
        }
    }
}
