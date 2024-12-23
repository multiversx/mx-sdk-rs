use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc::types::{RHListItem, RHListItemExec, TxEnv};

use crate::scenario_model::{BytesValue, CheckValue, TxExpect, TxResponse};

/// Verifies that transaction result error matches the given one.
///
/// Can only be used in tests and interactors, not available in contracts.
pub struct ExpectError<'a>(pub u64, pub &'a str);

impl<Env, Original> RHListItem<Env, Original> for ExpectError<'_>
where
    Env: TxEnv,
{
    type Returns = ();
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ExpectError<'_>
where
    Env: TxEnv<RHExpect = TxExpect>,
{
    fn item_tx_expect(&self, mut prev: TxExpect) -> TxExpect {
        prev.status = CheckValue::Equal(self.0.into());
        let expect_message_expr = BytesValue {
            value: self.1.to_string().into_bytes(),
            original: ValueSubTree::Str(format!("str:{}", self.1)),
        };
        prev.message = CheckValue::Equal(expect_message_expr);
        prev
    }

    fn item_process_result(self, _: &TxResponse) -> Self::Returns {}
}
