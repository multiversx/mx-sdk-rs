#![allow(deprecated)]

use super::{ScCallStep, ScDeployStep, ScQueryStep, TypedScCall, TypedScDeploy, TypedScQuery};
use crate::{
    api::StaticApi,
    multiversx_sc::{
        codec::TopEncodeMulti,
        types::{
            ContractCall, ContractCallNoPayment, ContractCallWithEgld,
            ContractCallWithEgldOrSingleEsdt, ContractCallWithMultiEsdt, ContractDeploy,
        },
    },
};

/// Converts a [`ContractCall`] or [`ContractDeploy`] into a scenario object that additonally
/// contains gas costs and transaction-related data.
#[deprecated(
    since = "0.42.0",
    note = "The recommended syntax is a variation of `sc_call` or `sc_deploy` with a scenario step built from the ContractCall."
)]
pub trait IntoBlockchainCall {
    type BlockchainCall;

    #[deprecated(
        since = "0.42.0",
        note = "The recommended syntax is a variation of `sc_call` or `sc_deploy` with a scenario step built from the ContractCall."
    )]
    fn into_blockchain_call(self) -> Self::BlockchainCall;
}

// implementing the trait for all ContractCall types explicitly
// otherwise the orphan rules kick in
macro_rules! impl_into_blockchain_call_cc {
    ($cc:ident) => {
        impl<OriginalResult> IntoBlockchainCall for $cc<StaticApi, OriginalResult>
        where
            OriginalResult: TopEncodeMulti,
        {
            type BlockchainCall = TypedScCall<OriginalResult>;

            fn into_blockchain_call(self) -> Self::BlockchainCall {
                ScCallStep::new().call(self).into()
            }
        }
    };
}

impl_into_blockchain_call_cc! {ContractCallNoPayment}
impl_into_blockchain_call_cc! {ContractCallWithEgld}
impl_into_blockchain_call_cc! {ContractCallWithEgldOrSingleEsdt}
impl_into_blockchain_call_cc! {ContractCallWithMultiEsdt}

impl<OriginalResult> IntoBlockchainCall for ContractDeploy<StaticApi, OriginalResult> {
    type BlockchainCall = TypedScDeploy<OriginalResult>;

    fn into_blockchain_call(self) -> Self::BlockchainCall {
        ScDeployStep::new().call(self)
    }
}

/// Converts a `ContractCall` into a scenario object that encodes a SC query.
pub trait IntoVMQuery {
    type VMQuery;

    fn into_vm_query(self) -> Self::VMQuery;
}

impl<CC> IntoVMQuery for CC
where
    CC: ContractCall<StaticApi>,
{
    type VMQuery = TypedScQuery<CC::OriginalResult>;
    fn into_vm_query(self) -> Self::VMQuery {
        ScQueryStep::default().call(self)
    }
}
