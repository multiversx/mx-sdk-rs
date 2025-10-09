use crate::{
    contract_base::SyncCallRawResult,
    types::{DeployRawResult, ManagedBuffer, ManagedVec, RHListItem, RHListItemExec, TxEnv},
};

/// Indicates that the raw result data will be returned.
pub struct ReturnsRawResult;

impl<Env, Original> RHListItem<Env, Original> for ReturnsRawResult
where
    Env: TxEnv,
{
    type Returns = ManagedVec<Env::Api, ManagedBuffer<Env::Api>>;
}

impl<Env, Original> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original> for ReturnsRawResult
where
    Env: TxEnv,
{
    fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Self::Returns {
        raw_result.0.clone()
    }
}

impl<Env, Original> RHListItemExec<DeployRawResult<Env::Api>, Env, Original> for ReturnsRawResult
where
    Env: TxEnv,
{
    fn item_process_result(self, raw_result: &DeployRawResult<Env::Api>) -> Self::Returns {
        raw_result.raw_results.clone()
    }
}
