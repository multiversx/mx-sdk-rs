use crate::{
    api::{self, CallTypeApi, ManagedTypeApi},
    contract_base::{BlockchainWrapper, SendRawWrapper},
    types::{
        BigUint, CodeMetadata, EgldOrEsdtTokenPayment, EgldOrMultiEsdtPayment, EgldPayment,
        EsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedOption, ManagedVec,
        MultiEsdtPayment,
    },
};
use alloc::boxed::Box;
use multiversx_sc_codec::TopEncodeMulti;

use super::{
    contract_call_exec::UNSPECIFIED_GAS_LIMIT, contract_call_trait::ContractCallBase,
    AnnotatedValue, AsyncCall, Code, ContractCallNoPayment, ContractCallWithEgld, ContractDeploy,
    DeployCall, ExplicitGas, FromSource, FunctionCall, ManagedArgBuffer, OriginalResultMarker,
    RHList, RHListAppendNoRet, RHListAppendRet, RHListItem, TxCodeSource, TxCodeValue, TxData,
    TxDataFunctionCall, TxEnv, TxFrom, TxFromSourceValue, TxFromSpecified, TxGas, TxPayment,
    TxPaymentEgldOnly, TxProxyTrait, TxResultHandler, TxScEnv, TxTo, TxToSpecified,
};

/// Defines how a payment transforms a transaction,
/// e.g. from ESDT transfer to ESDTTransfer builtin function.
pub trait TxPaymentNormalize<Env, From, To>: TxPayment<Env>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R;
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for ()
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| f(to_addr, &BigUint::zero(), &fc))
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for EgldPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| f(to_addr, &self.value, &fc))
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for EsdtTokenPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| {
            if self.token_nonce == 0 {
                let fc_conv = fc.convert_to_single_transfer_fungible_call(self);
                f(to_addr, &BigUint::zero(), &fc_conv)
            } else {
                let fc_conv = fc.convert_to_single_transfer_nft_call(to_addr, self);
                f(&from.resolve_address(env), &BigUint::zero(), &fc_conv)
            }
        })
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for MultiEsdtPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| {
            let fc_conv = fc.convert_to_multi_transfer_esdt_call(to_addr, self);
            f(&from.resolve_address(env), &BigUint::zero(), &fc_conv)
        })
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for EgldOrEsdtTokenPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        self.map_egld_or_esdt(
            (to, fc, f),
            |(to, fc, f), amount| EgldPayment::from(amount).with_normalized(env, from, to, fc, f),
            |(to, fc, f), esdt_payment| esdt_payment.with_normalized(env, from, to, fc, f),
        )
    }
}

impl<Env, From, To> TxPaymentNormalize<Env, From, To> for EgldOrMultiEsdtPayment<Env::Api>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
{
    fn with_normalized<F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, &FunctionCall<Env::Api>) -> R,
    {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                EgldPayment::from(egld_amount).with_normalized(env, from, to, fc, f)
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.with_normalized(env, from, to, fc, f)
            },
        }
    }
}
