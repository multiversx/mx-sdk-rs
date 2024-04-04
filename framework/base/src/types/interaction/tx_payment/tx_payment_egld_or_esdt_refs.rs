use alloc::borrow::ToOwned;

use crate::{
    api::ManagedTypeApi,
    contract_base::SendRawWrapper,
    types::{
        AnnotatedValue, BigUint, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment,
        EgldOrEsdtTokenPaymentRefs, EgldOrMultiEsdtPayment, EsdtTokenPayment, EsdtTokenPaymentRefs,
        ManagedAddress, ManagedType, ManagedVec, MultiEsdtPayment, TxFrom, TxToSpecified,
    },
};

use super::{
    AnnotatedEgldPayment, Egld, FullPaymentData, FunctionCall, TxEgldValue, TxEnv, TxPayment,
};

impl<'a, Env> TxPayment<Env> for EgldOrEsdtTokenPaymentRefs<'a, Env::Api>
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
        self.map_egld_or_esdt(
            fc,
            |fc, amount| Egld(amount).perform_transfer_execute(env, to, gas_limit, fc),
            |fc, esdt_payment| esdt_payment.perform_transfer_execute(env, to, gas_limit, fc),
        )
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
        self.map_egld_or_esdt(
            (to, fc, f),
            |(to, fc, f), amount| Egld(amount).with_normalized(env, from, to, fc, f),
            |(to, fc, f), esdt_payment| esdt_payment.with_normalized(env, from, to, fc, f),
        )
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        self.map_egld_or_esdt(
            (),
            |(), amount| TxPayment::<Env>::into_full_payment_data(Egld(amount), env),
            |(), esdt_payment| TxPayment::<Env>::into_full_payment_data(esdt_payment, env),
        )
    }
}
