use multiversx_sc::types::{RHListItem, RHListItemExec, TxEnv};

use crate::scenario_model::{CheckValue, TxExpect, TxResponse};

/// Indicates that the error status will be returned.
///
/// Can only be used in tests and interactors, not available in contracts.
pub struct ReturnsMessage;

impl<Env, Original> RHListItem<Env, Original> for ReturnsMessage
where
    Env: TxEnv,
{
    type Returns = String;
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsMessage
where
    Env: TxEnv<RHExpect = TxExpect>,
{
    fn item_tx_expect(&self, mut prev: TxExpect) -> TxExpect {
        prev.message = CheckValue::Star;
        prev
    }

    fn item_process_result(self, raw_result: &TxResponse) -> Self::Returns {
        raw_result.tx_error.message.clone()
    }
}
