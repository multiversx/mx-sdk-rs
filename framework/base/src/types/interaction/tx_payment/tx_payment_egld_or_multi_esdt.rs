use alloc::borrow::ToOwned;

use crate::{
    api::ManagedTypeApi,
    contract_base::SendRawWrapper,
    proxy_imports::{AnnotatedValue, EgldOrMultiEsdtPayment, ManagedType},
    types::{
        BigUint, EsdtTokenPayment, EsdtTokenPaymentRefs, ManagedAddress, ManagedVec,
        MultiEsdtPayment, TxFrom, TxToSpecified,
    },
};

use super::{
    AnnotatedEgldPayment, Egld, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment,
};

impl<Env> TxPayment<Env> for EgldOrMultiEsdtPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.is_empty()
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                Egld(egld_amount).perform_transfer_execute(env, to, gas_limit, fc)
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.perform_transfer_execute(env, to, gas_limit, fc)
            },
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
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                Egld(egld_amount).with_normalized(env, from, to, fc, f)
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.with_normalized(env, from, to, fc, f)
            },
        }
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                TxPayment::<Env>::into_full_payment_data(Egld(egld_amount), env)
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                TxPayment::<Env>::into_full_payment_data(multi_esdt_payment, env)
            },
        }
    }
}
