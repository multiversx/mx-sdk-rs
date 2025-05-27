use multiversx_sc::types::{RHListItem, RHListItemExec, TxEnv};

use crate::scenario_model::TxResponse;

/// Indicates that the newly deployed address will be returned after a deploy.
pub struct ReturnsGasUsed;

impl<Env, Original> RHListItem<Env, Original> for ReturnsGasUsed
where
    Env: TxEnv,
{
    type Returns = u64;
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsGasUsed
where
    Env: TxEnv,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        tx_response.gas_used
    }
}
