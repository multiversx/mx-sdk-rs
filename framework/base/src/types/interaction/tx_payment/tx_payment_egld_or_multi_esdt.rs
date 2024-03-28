use alloc::borrow::ToOwned;

use crate::{
    api::ManagedTypeApi,
    contract_base::SendRawWrapper,
    proxy_imports::{
        AnnotatedValue, EgldOrMultiEsdtPayment, EgldOrMultiEsdtPaymentRefs, ManagedType,
    },
    types::{
        BigUint, EsdtTokenPayment, EsdtTokenPaymentRefs, ManagedAddress, ManagedVec,
        MultiEsdtPayment, TxFrom, TxToSpecified,
    },
};

use super::{AnnotatedEgldPayment, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment};

impl<'a, Env> TxPayment<Env> for EgldOrMultiEsdtPaymentRefs<'a, Env::Api>
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
            EgldOrMultiEsdtPaymentRefs::Egld(egld_amount) => {
                let _ = SendRawWrapper::<Env::Api>::new().direct_egld_execute(
                    to,
                    egld_amount,
                    gas_limit,
                    &fc.function_name,
                    &fc.arg_buffer,
                );
            },
            EgldOrMultiEsdtPaymentRefs::MultiEsdt(multi_esdt_payment) => {
                let _ = SendRawWrapper::<Env::Api>::new().multi_esdt_transfer_execute(
                    to,
                    multi_esdt_payment,
                    gas_limit,
                    &fc.function_name,
                    &fc.arg_buffer,
                );
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
            EgldOrMultiEsdtPaymentRefs::Egld(egld_amount) => {
                to.with_address_ref(env, |to_addr| f(to_addr, &BigUint::zero(), &fc))
            },
            EgldOrMultiEsdtPaymentRefs::MultiEsdt(multi_esdt_payment) => {
                to.with_address_ref(env, |to_addr| {
                    let fc_conv =
                        fc.convert_to_multi_transfer_esdt_call(to_addr, multi_esdt_payment);
                    f(to_addr, &BigUint::zero(), &fc_conv)
                })
            },
        }
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        match self {
            EgldOrMultiEsdtPaymentRefs::Egld(egld_amount) => FullPaymentData {
                egld: Some(AnnotatedEgldPayment::new_egld(egld_amount.into_value(env))),
                multi_esdt: ManagedVec::new(),
            },
            EgldOrMultiEsdtPaymentRefs::MultiEsdt(multi_esdt_payment) => FullPaymentData {
                egld: None,
                multi_esdt: multi_esdt_payment.clone(),
            },
        }
    }
}
