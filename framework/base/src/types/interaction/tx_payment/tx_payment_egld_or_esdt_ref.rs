use alloc::borrow::ToOwned;

use crate::{
    api::ManagedTypeApi,
    contract_base::SendRawWrapper,
    proxy_imports::EgldOrEsdtTokenIdentifier,
    types::{
        AnnotatedValue, BigUint, EgldOrEsdtTokenPayment, EgldOrMultiEsdtPayment, EsdtTokenPayment,
        EsdtTokenPaymentRefs, ManagedAddress, ManagedType, ManagedVec, MultiEsdtPayment, TxFrom,
        TxToSpecified,
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
            (to, fc),
            |(to, fc), amount| Egld(amount).perform_transfer_execute(env, to, gas_limit, fc),
            |(to, fc), esdt_payment| esdt_payment.perform_transfer_execute(env, to, gas_limit, fc),
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

pub struct EgldOrEsdtTokenPaymentRefs<'a, M: ManagedTypeApi> {
    pub token_identifier: &'a EgldOrEsdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: &'a BigUint<M>,
}

impl<M: ManagedTypeApi> EgldOrEsdtTokenPayment<M> {
    pub fn as_refs(&self) -> EgldOrEsdtTokenPaymentRefs<'_, M> {
        EgldOrEsdtTokenPaymentRefs {
            token_identifier: &self.token_identifier,
            token_nonce: self.token_nonce,
            amount: &self.amount,
        }
    }
}

impl<'a, M: ManagedTypeApi> EgldOrEsdtTokenPaymentRefs<'a, M> {
    pub fn to_owned_payment(&self) -> EgldOrEsdtTokenPayment<M> {
        EgldOrEsdtTokenPayment {
            token_identifier: self.token_identifier.clone(),
            token_nonce: self.token_nonce,
            amount: self.amount.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.amount == &BigUint::zero()
    }

    pub fn map_egld_or_esdt<Context, D, F, U>(self, context: Context, for_egld: D, for_esdt: F) -> U
    where
        D: FnOnce(Context, &BigUint<M>) -> U,
        F: FnOnce(Context, EgldOrEsdtTokenPaymentRefs<M>) -> U,
    {
        if self.token_identifier.data.is_some() {
            let token_identifier = unsafe { self.token_identifier.clone().data.unwrap_no_check() };
            for_esdt(
                context,
                EgldOrEsdtTokenPaymentRefs {
                    token_identifier: &EgldOrEsdtTokenIdentifier::esdt(token_identifier),
                    token_nonce: self.token_nonce,
                    amount: self.amount,
                },
            )
        } else {
            for_egld(context, self.amount)
        }
    }
}
