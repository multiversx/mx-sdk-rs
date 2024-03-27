use multiversx_sc::types::{RHListItem, RHListItemExec, TxEnv};

use crate::scenario_model::{CheckValue, TxExpect, TxResponse};

/// Verifies that transaction result status matches the given one.
///
/// Can only be used in tests and interactors, not available in contracts.
pub struct ExpectStatus(pub u64);

impl<Env, Original> RHListItem<Env, Original> for ExpectStatus
where
    Env: TxEnv,
{
    type Returns = ();
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ExpectStatus
where
    Env: TxEnv<RHExpect = TxExpect>,
{
    fn item_tx_expect(&self, mut prev: TxExpect) -> TxExpect {
        prev.status = CheckValue::Equal(self.0.into());
        prev
    }

    fn item_process_result(self, _: &TxResponse) -> Self::Returns {}
}
