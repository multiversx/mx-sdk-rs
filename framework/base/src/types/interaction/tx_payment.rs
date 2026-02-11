mod tx_payment_egld;
mod tx_payment_egld_or_esdt;
mod tx_payment_egld_or_esdt_refs;
mod tx_payment_egld_or_multi_esdt;
mod tx_payment_egld_or_multi_esdt_refs;
mod tx_payment_egld_value;
mod tx_payment_multi_egld_or_esdt;
mod tx_payment_multi_esdt;
mod tx_payment_multi_transfer_marker;
mod tx_payment_none;
mod tx_payment_not_payable;
mod tx_payment_payment;
mod tx_payment_payment_option;
mod tx_payment_payment_ref;
mod tx_payment_payment_refs;
mod tx_payment_single_esdt;
mod tx_payment_single_esdt_ref;
mod tx_payment_triple;
mod tx_payment_vec_ref;

pub use tx_payment_egld::{Egld, EgldPayment};
pub use tx_payment_egld_value::TxEgldValue;
pub use tx_payment_multi_esdt::TxPaymentMultiEsdt;
pub use tx_payment_not_payable::NotPayable;

use crate::{
    api::{CallTypeApi, ManagedTypeApi, quick_signal_error},
    contract_base::TransferExecuteFailed,
    err_msg,
    types::{BigUint, ManagedAddress, ManagedBuffer, MultiEgldOrEsdtPayment},
};

use super::{AnnotatedValue, FunctionCall, TxEnv, TxFrom, TxToSpecified};

/// Describes a payment that is part of a transaction.
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot be used as payment (does not implement `TxPayment<{Env}>`)",
    label = "not a valid payment type",
    note = "there are multiple ways to specify the transaction payment, but `{Self}` is not one of them"
)]
pub trait TxPayment<Env>: Sized
where
    Env: TxEnv,
{
    /// Returns true if payment indicates transfer of either non-zero EGLD or ESDT amounts.
    fn is_no_payment(&self, env: &Env) -> bool;

    /// Transfer-execute calls have different APIs for different payments types.
    /// This method selects between them.
    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed>;

    /// Shortcut for doing direct transfers.
    ///
    /// It is relevant with EGLD: it is simpler to perform direct EGLD transfers,
    /// instead of going via multi-transfer.
    fn perform_transfer_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        self.perform_transfer_execute_fallible(env, to, 0, FunctionCall::empty())
    }

    /// Allows transfer-execute without payment.
    fn perform_transfer_execute_legacy(
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
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R;

    /// Payment data to be used by the testing framework. Will be refactored.
    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api>;
}

/// Trait for composing multiple payment objects into a single payment.
///
/// This trait allows combining different types of payments (EGLD, ESDT, multi-transfers)
/// into a unified payment structure. It's useful when building complex transactions that
/// involve multiple token transfers.
///
/// # Examples
///
/// ```ignore
/// // Composing EGLD payment with ESDT payment
/// let payment = egld_payment.compose(esdt_payment);
///
/// // Composing multiple ESDT payments
/// let multi_payment = esdt1.compose(esdt2).compose(esdt3);
/// ```
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot be composed with `{Rhs}` (does not implement `TxPaymentCompose<{Env}, {Rhs}>`)",
    label = "payment composition not supported",
    note = "only certain payment types can be composed together - check that both types implement the required traits"
)]
pub trait TxPaymentCompose<Env, Rhs>: TxPayment<Env>
where
    Env: TxEnv,
    Rhs: TxPayment<Env>,
{
    /// The resulting payment type after composition.
    type Output: TxPayment<Env>;

    /// Combines this payment with another payment, returning a new payment object
    /// that represents both transfers.
    fn compose(self, rhs: Rhs) -> Self::Output;
}

impl<Env, P> TxPaymentCompose<Env, P> for ()
where
    Env: TxEnv,
    P: TxPayment<Env>,
{
    type Output = P;

    fn compose(self, rhs: P) -> Self::Output {
        rhs
    }
}

/// Marker trait that indicates that payment field contains no payment.
///
/// Implemented by `()` and `NotPayable`.
pub trait TxNoPayment<Env>: TxPayment<Env>
where
    Env: TxEnv,
{
}

/// Marks a payment object that only contains EGLD or nothing at all.
pub trait TxPaymentEgldOnly<Env>: TxPayment<Env> + AnnotatedValue<Env, BigUint<Env::Api>>
where
    Env: TxEnv,
{
    #[inline]
    fn with_egld_value<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        self.with_value_ref(env, f)
    }

    fn into_egld_payment(self, env: &Env) -> BigUint<Env::Api> {
        self.into_value(env)
    }
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
    pub multi_esdt: MultiEgldOrEsdtPayment<Api>,
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

#[allow(unused)]
pub(crate) fn transfer_execute_failed_error<Api: CallTypeApi>(
    result: Result<(), TransferExecuteFailed>,
) {
    if result.is_err() {
        quick_signal_error::<Api>(err_msg::TRANSFER_EXECUTE_FAILED);
    }
}
