use crate::{
    api::{self, CallTypeApi, ManagedTypeApi},
    contract_base::{BlockchainWrapper, SendRawWrapper},
    types::{
        BigUint, CodeMetadata, EgldOrEsdtTokenPayment, EgldOrMultiEsdtPayment, EsdtTokenPayment,
        ManagedAddress, ManagedBuffer, ManagedOption, ManagedVec, MultiEsdtPayment,
    },
};
use alloc::boxed::Box;
use multiversx_sc_codec::TopEncodeMulti;

use super::{
    contract_call_exec::UNSPECIFIED_GAS_LIMIT, contract_call_trait::ContractCallBase,
    AnnotatedEgldPayment, AnnotatedValue, AsyncCall, Code, ContractCallNoPayment,
    ContractCallWithEgld, ContractDeploy, DeployCall, ExplicitGas, FromSource, FullPaymentData,
    FunctionCall, ManagedArgBuffer, OriginalResultMarker, RHList, RHListAppendNoRet,
    RHListAppendRet, RHListItem, TxCodeSource, TxCodeValue, TxData, TxDataFunctionCall, TxEnv,
    TxFrom, TxFromSourceValue, TxFromSpecified, TxGas, TxPayment, TxProxyTrait, TxResultHandler,
    TxScEnv, TxTo, TxToSpecified,
};

pub trait TxEgldValue<Env>: AnnotatedValue<Env, BigUint<Env::Api>>
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R;
}

impl<Env> TxEgldValue<Env> for BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> TxEgldValue<Env> for &BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(*self)
    }
}

impl<Env> TxEgldValue<Env> for u64
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(&BigUint::<Env::Api>::from(*self))
    }
}
