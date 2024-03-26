use multiversx_sc::types::{RHListItem, RHListItemExec, TxEnv};

use crate::scenario_model::{CheckValue, TxExpect, TxResponse, U64Value};

/// Indicates that the error status will be returned.
///
/// Can only be used in tests and interactors, not available in contracts.
pub struct ReturnsStatus;

impl<Env, Original> RHListItem<Env, Original> for ReturnsStatus
where
    Env: TxEnv,
{
    type Returns = u64;
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsStatus
where
    Env: TxEnv<RHExpect = TxExpect>,
{
    fn item_tx_expect(&self, mut prev: TxExpect) -> TxExpect {
        if let CheckValue::Equal(U64Value {
            value: 0,
            original: _,
        }) = prev.status
        {
            prev.status = CheckValue::Star;
        }

        prev
    }

    fn item_process_result(self, raw_result: &TxResponse) -> Self::Returns {
        raw_result.tx_error.status
    }
}
