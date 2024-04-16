mod tx_payment_egld;
mod tx_payment_egld_or_esdt;
mod tx_payment_egld_or_esdt_refs;
mod tx_payment_egld_or_multi_esdt;
mod tx_payment_egld_or_multi_esdt_ref;
mod tx_payment_egld_value;
mod tx_payment_multi_esdt;
mod tx_payment_none;
mod tx_payment_single_esdt;
mod tx_payment_single_esdt_ref;
mod tx_payment_single_esdt_triple;

pub use tx_payment_egld::{Egld, EgldPayment};
pub use tx_payment_egld_value::TxEgldValue;
pub use tx_payment_multi_esdt::TxPaymentMultiEsdt;

use crate::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress, ManagedBuffer, ManagedRef, MultiEsdtPayment},
};

use super::{AnnotatedValue, FunctionCall, TxEnv, TxFrom, TxToSpecified};

/// Describes a payment that is part of a transaction.
pub trait TxPayment<Env>
where
    Env: TxEnv,
{
    /// Returns true if payment indicates transfer of either non-zero EGLD or ESDT amounts.
    fn is_no_payment(&self, env: &Env) -> bool;

    /// Transfer-execute calls have different APIs for different payments types.
    /// This method selects between them.
    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    );

    /// Converts an ESDT call to a built-in function call, if necessary.
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
        F: FnOnce(
            ManagedRef<'_, Env::Api, ManagedAddress<Env::Api>>,
            ManagedRef<'_, Env::Api, BigUint<Env::Api>>,
            &FunctionCall<Env::Api>,
        ) -> R;

    /// Payment data to be used by the testing framework. Will be refactored.
    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api>;
}

/// Marks a payment object that only contains EGLD or nothing at all.
pub trait TxPaymentEgldOnly<Env>: TxPayment<Env> + AnnotatedValue<Env, BigUint<Env::Api>>
where
    Env: TxEnv,
{
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
