use multiversx_sc::types::{RHListItem, RHListItemExec, TxEnv};

use crate::scenario_model::TxResponse;

/// Indicates that the error status will be returned.
pub struct ReturnsMessage;

impl<Env, Original> RHListItem<Env, Original> for ReturnsMessage
where
    Env: TxEnv,
{
    type Returns = String;
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsMessage
where
    Env: TxEnv,
{
    fn is_error_handled(&self) -> bool {
        true
    }

    fn item_process_result(self, raw_result: &TxResponse) -> Self::Returns {
        raw_result.tx_error.message.clone()
    }
}
