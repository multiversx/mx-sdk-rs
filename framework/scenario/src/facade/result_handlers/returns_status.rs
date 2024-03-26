use multiversx_sc::types::{RHListItem, RHListItemExec, TxEnv};

use crate::scenario_model::TxResponse;

/// Indicates that the error status will be returned.
pub struct ReturnsStatus;

impl<Env, Original> RHListItem<Env, Original> for ReturnsStatus
where
    Env: TxEnv,
{
    type Returns = u64;
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsStatus
where
    Env: TxEnv,
{
    fn is_error_handled(&self) -> bool {
        true
    }

    fn item_process_result(self, raw_result: &TxResponse) -> Self::Returns {
        raw_result.tx_error.status
    }
}
