use crate::{
    contract_base::TransferExecuteFailed,
    types::{BigUint, ManagedAddress, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxNoPayment, TxPayment, TxPaymentEgldOnly};

/// Transaction marker, which indicates that a transaction should never have any payment added to it.
///
/// The implementation is completely identical to the empty payment `()`,
/// the only difference is that the payment methods in `Tx` can only be called on top of `()` payment, not `NotPayable`.
///
/// So basically, `NotPayable` acts as a seal, preventing further payments to be added.
pub struct NotPayable;

impl<Env> TxPayment<Env> for NotPayable
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        true
    }

    #[inline]
    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        ().perform_transfer_execute_fallible(env, to, gas_limit, fc)
    }

    #[inline]
    fn perform_transfer_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        ().perform_transfer_fallible(env, to)
    }

    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        ().perform_transfer_execute_legacy(env, to, gas_limit, fc);
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
        ().with_normalized(env, from, to, fc, f)
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData::default()
    }
}

impl<Env> TxNoPayment<Env> for NotPayable where Env: TxEnv {}

impl<Env> TxPaymentEgldOnly<Env> for NotPayable where Env: TxEnv {}
