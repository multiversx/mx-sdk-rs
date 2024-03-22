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
    RHListAppendRet, RHListItem, TxCodeSource, TxCodeValue, TxData, TxDataFunctionCall,
    TxEgldValue, TxEnv, TxFrom, TxFromSourceValue, TxFromSpecified, TxGas, TxPayment, TxProxyTrait,
    TxResultHandler, TxScEnv, TxTo, TxToSpecified,
};

/// Indicates the EGLD payment in a transaction.
pub struct Egld<EgldValue>(pub EgldValue);

pub type EgldPayment<Api> = Egld<BigUint<Api>>;

impl<Env, EgldValue> TxPayment<Env> for Egld<EgldValue>
where
    Env: TxEnv,
    EgldValue: TxEgldValue<Env>,
{
    fn is_no_payment(&self) -> bool {
        self.0.with_egld_value(|egld_value| egld_value == &0u32)
    }

    fn perform_transfer_execute(
        self,
        _env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.0.with_egld_value(|egld_value| {
            let _ = SendRawWrapper::<Env::Api>::new().direct_egld_execute(
                to,
                egld_value,
                gas_limit,
                &fc.function_name,
                &fc.arg_buffer,
            );
        })
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: Some(AnnotatedEgldPayment::new_egld(self.0.into_value(env))),
            multi_esdt: ManagedVec::new(),
        }
    }
}

/// Marks a payment object that only contains EGLD or nothing at all.
pub trait TxPaymentEgldOnly<Env>: TxPayment<Env>
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R;

    fn into_egld_payment(self, env: &Env) -> BigUint<Env::Api>;
}

impl<Env> TxPaymentEgldOnly<Env> for ()
where
    Env: TxEnv,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(&BigUint::zero())
    }

    fn into_egld_payment(self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::zero()
    }
}

impl<Env, EgldValue> TxPaymentEgldOnly<Env> for Egld<EgldValue>
where
    Env: TxEnv,
    EgldValue: TxEgldValue<Env>,
{
    fn with_egld_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        self.0.with_egld_value(f)
    }

    fn into_egld_payment(self, env: &Env) -> BigUint<Env::Api> {
        self.0.into_value(env)
    }
}
