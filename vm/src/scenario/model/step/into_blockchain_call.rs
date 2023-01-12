use multiversx_sc::types::{ContractCall, ContractDeploy};

use crate::DebugApi;

use super::{ScCallStep, ScDeployStep, ScQueryStep, TypedScCall, TypedScDeploy, TypedScQuery};

/// Converts a `ContractCall` or `ContractDeploy` into a scenario object that additonally
/// contains gas costs and transaction-related data.
pub trait IntoBlockchainCall {
    type BlockchainCall;

    fn into_blockchain_call(self) -> Self::BlockchainCall;
}

impl<CC> IntoBlockchainCall for CC
where
    CC: ContractCall<DebugApi>,
{
    type BlockchainCall = TypedScCall<CC::OriginalResult>;

    fn into_blockchain_call(self) -> Self::BlockchainCall {
        ScCallStep::new().call(self).into()
    }
}

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
