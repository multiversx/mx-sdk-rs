use crate::types::{DeployRawResult, ManagedAddress, ManagedBuffer, ManagedVec, TxEnv};

use super::{RHListItem, RHListItemExec};

pub struct ReturnsNewAddress;

impl<Env, Original> RHListItem<Env, Original> for ReturnsNewAddress
where
    Env: TxEnv,
{
    type Returns = ManagedAddress<Env::Api>;
}

impl<Env, Original> RHListItemExec<DeployRawResult<Env::Api>, Env, Original> for ReturnsNewAddress
where
    Env: TxEnv,
{
    fn item_process_result(self, raw_result: &DeployRawResult<Env::Api>) -> Self::Returns {
        raw_result.new_address.clone()
    }
}
