use elrond_wasm::types::{ContractCall, ContractDeploy};

use crate::DebugApi;

use super::{ScCallStep, ScDeployStep, ScQueryStep, TypedScCall, TypedScDeploy, TypedScQuery};

/// Converts a `ContractCall` or `ContractDeploy` into a mandos object that additonally
/// contains gas costs and transaction-related data.
pub trait IntoBlockchainCall {
    type BlockchainCall;

    fn into_blockchain_call(self) -> Self::BlockchainCall;
}

impl<OriginalResult> IntoBlockchainCall for ContractCall<DebugApi, OriginalResult> {
    type BlockchainCall = TypedScCall<OriginalResult>;

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

/// Converts a `ContractCall` into a mandos object that encodes a SC query.
pub trait IntoVMQuery {
    type VMQuery;

    fn into_vm_query(self) -> Self::VMQuery;
}

impl<OriginalResult> IntoVMQuery for ContractCall<DebugApi, OriginalResult> {
    type VMQuery = TypedScQuery<OriginalResult>;
    fn into_vm_query(self) -> Self::VMQuery {
        ScQueryStep::default().call(self).into()
    }
}
