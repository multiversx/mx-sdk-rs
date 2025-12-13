use crate::{
    contract_base::TransferExecuteFailed,
    types::{BigUint, ManagedAddress, MultiTransfer, PaymentVec, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env, P> TxPayment<Env> for P
where
    Env: TxEnv,
    P: AsRef<PaymentVec<Env::Api>>,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        let pv = self.as_ref();
        pv.is_empty()
    }

    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        let pv = self.as_ref();
        match pv.len() {
            0 => ().perform_transfer_execute_fallible(env, to, gas_limit, fc),
            1 => pv
                .get(0)
                .perform_transfer_execute_fallible(env, to, gas_limit, fc),
            _ => MultiTransfer(pv).perform_transfer_execute_fallible(env, to, gas_limit, fc),
        }
    }

    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        let pv = self.as_ref();
        match pv.len() {
            0 => ().perform_transfer_execute_legacy(env, to, gas_limit, fc),
            1 => pv
                .get(0)
                .perform_transfer_execute_legacy(env, to, gas_limit, fc),
            _ => MultiTransfer(pv).perform_transfer_execute_legacy(env, to, gas_limit, fc),
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
        let pv = self.as_ref();
        match pv.len() {
            0 => ().with_normalized(env, from, to, fc, f),
            1 => pv.get(0).with_normalized(env, from, to, fc, f),
            _ => MultiTransfer(pv).with_normalized(env, from, to, fc, f),
        }
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        let pv = self.as_ref();
        match pv.len() {
            0 => ().into_full_payment_data(env),
            1 => pv.get(0).into_full_payment_data(env),
            _ => MultiTransfer(pv).into_full_payment_data(env),
        }
    }
}
