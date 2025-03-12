use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc::types::{RHListItem, RHListItemExec, TxEnv};

use crate::scenario_model::{BytesValue, CheckValue, TxExpect, TxResponse, U64Value};

/// Verifies that transaction result message matches the given one.
///
/// Can only be used in tests and interactors, not available in contracts.
pub struct ExpectMessage<'a>(pub &'a str);

impl<Env, Original> RHListItem<Env, Original> for ExpectMessage<'_>
where
    Env: TxEnv,
{
    type Returns = ();
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ExpectMessage<'_>
where
    Env: TxEnv<RHExpect = TxExpect>,
{
    fn item_preprocessing(&self, mut prev: TxExpect) -> TxExpect {
        if prev.status.is_equal_to(U64Value::empty()) {
            prev.status = CheckValue::Star;
        }

        let expect_message_expr = BytesValue {
            value: self.0.to_string().into_bytes(),
            original: ValueSubTree::Str(format!("str:{}", self.0)),
        };
        prev.message = CheckValue::Equal(expect_message_expr);
        prev
    }

    fn item_process_result(self, _: &TxResponse) -> Self::Returns {}
}
