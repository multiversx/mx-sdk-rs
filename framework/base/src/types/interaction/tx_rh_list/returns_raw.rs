use crate::{
    proxy_imports::SyncCallRawResult,
    types::{DeployRawResult, ManagedAddress, ManagedBuffer, ManagedVec, TxEnv},
};

use super::{RHListItem, RHListItemExec};

pub struct ReturnsRaw;

impl<Env, Original> RHListItem<Env, Original> for ReturnsRaw
where
    Env: TxEnv,
{
    type Returns = ManagedVec<Env::Api, ManagedBuffer<Env::Api>>;
}

impl<Env, Original> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original> for ReturnsRaw
where
    Env: TxEnv,
{
    fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Self::Returns {
        raw_result.0.clone()
    }
}

impl<Env, Original> RHListItemExec<DeployRawResult<Env::Api>, Env, Original> for ReturnsRaw
where
    Env: TxEnv,
{
    fn item_process_result(self, raw_result: &DeployRawResult<Env::Api>) -> Self::Returns {
        raw_result.raw_results.clone()
    }
}
