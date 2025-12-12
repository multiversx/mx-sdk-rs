use crate::{
    contract_base::TransferExecuteFailed,
    types::{BigUint, EgldOrMultiEsdtPaymentRefs, ManagedAddress, TxFrom, TxToSpecified},
};

use super::{Egld, FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for EgldOrMultiEsdtPaymentRefs<'_, Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.is_empty()
    }

    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        match self {
            EgldOrMultiEsdtPaymentRefs::Egld(egld_amount) => {
                Egld(egld_amount).perform_transfer_execute_fallible(env, to, gas_limit, fc)
            }
            EgldOrMultiEsdtPaymentRefs::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.perform_transfer_execute_fallible(env, to, gas_limit, fc)
            }
        }
    }

    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        match self {
            EgldOrMultiEsdtPaymentRefs::Egld(egld_amount) => {
                Egld(egld_amount).perform_transfer_execute_legacy(env, to, gas_limit, fc)
            }
            EgldOrMultiEsdtPaymentRefs::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.perform_transfer_execute_legacy(env, to, gas_limit, fc)
            }
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
        match self {
            EgldOrMultiEsdtPaymentRefs::Egld(egld_amount) => {
                Egld(egld_amount).with_normalized(env, from, to, fc, f)
            }
            EgldOrMultiEsdtPaymentRefs::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.with_normalized(env, from, to, fc, f)
            }
        }
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        match self {
            EgldOrMultiEsdtPaymentRefs::Egld(egld_amount) => {
                Egld(egld_amount).into_full_payment_data(env)
            }
            EgldOrMultiEsdtPaymentRefs::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.into_full_payment_data(env)
            }
        }
    }
}
