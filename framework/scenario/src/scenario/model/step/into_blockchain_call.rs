use crate::multiversx_sc::types::{
    ContractCallNoPayment, ContractCallWithEgld, ContractCallWithEgldOrSingleEsdt,
    ContractCallWithMultiEsdt, ContractDeploy,
};

use multiversx_chain_vm::{
    multiversx_sc::{codec::TopEncodeMulti, types::ContractCall},
    DebugApi,
};

use super::{ScCallStep, ScDeployStep, ScQueryStep, TypedScCall, TypedScDeploy, TypedScQuery};

/// Converts a `ContractCall` or `ContractDeploy` into a scenario object that additonally
/// contains gas costs and transaction-related data.
pub trait IntoBlockchainCall {
    type BlockchainCall;

    fn into_blockchain_call(self) -> Self::BlockchainCall;
}

// implementing the trait for all ContractCall types explicitly
// otherwise the orphan rules kick in
macro_rules! impl_into_blockchain_call_cc {
    ($cc:ident) => {
        impl<OriginalResult> IntoBlockchainCall for $cc<DebugApi, OriginalResult>
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

impl<OriginalResult> IntoBlockchainCall for ContractDeploy<DebugApi, OriginalResult> {
    type BlockchainCall = TypedScDeploy<OriginalResult>;

    fn into_blockchain_call(self) -> Self::BlockchainCall {
        ScDeployStep::new().call(self).into()
    }
}

/// Converts a `ContractCall` into a scenario object that encodes a SC query.
pub trait IntoVMQuery {
    type VMQuery;

    fn into_vm_query(self) -> Self::VMQuery;
}

impl<CC> IntoVMQuery for CC
where
    CC: ContractCall<DebugApi>,
{
    type VMQuery = TypedScQuery<CC::OriginalResult>;
    fn into_vm_query(self) -> Self::VMQuery {
        ScQueryStep::default().call(self).into()
    }
}
