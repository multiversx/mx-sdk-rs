use crate::types::{DeployRawResult, ManagedAddress, RHListItem, RHListItemExec, TxEnv};

/// Indicates that the newly deployed address will be returned after a deploy as a ManagedAddress.
pub struct ReturnsNewManagedAddress;

impl<Env, Original> RHListItem<Env, Original> for ReturnsNewManagedAddress
where
    Env: TxEnv,
{
    type Returns = ManagedAddress<Env::Api>;
}

impl<Env, Original> RHListItemExec<DeployRawResult<Env::Api>, Env, Original>
    for ReturnsNewManagedAddress
where
    Env: TxEnv,
{
    fn item_process_result(self, raw_result: &DeployRawResult<Env::Api>) -> Self::Returns {
        raw_result.new_address.clone()
    }
}
