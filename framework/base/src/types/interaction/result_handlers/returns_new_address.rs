use crate::types::{DeployRawResult, RHListItem, RHListItemExec, TxEnv, heap::Address};

/// Indicates that the newly deployed address will be returned after a deploy.
pub struct ReturnsNewAddress;

impl<Env, Original> RHListItem<Env, Original> for ReturnsNewAddress
where
    Env: TxEnv,
{
    type Returns = Address;
}

impl<Env, Original> RHListItemExec<DeployRawResult<Env::Api>, Env, Original> for ReturnsNewAddress
where
    Env: TxEnv,
{
    fn item_process_result(self, raw_result: &DeployRawResult<Env::Api>) -> Self::Returns {
        raw_result.new_address.to_address()
    }
}
