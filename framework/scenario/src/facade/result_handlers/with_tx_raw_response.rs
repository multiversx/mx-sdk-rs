use multiversx_sc::{
    codec::TopDecodeMulti,
    types::{RHListItem, RHListItemExec, TxEnv},
};

use crate::scenario_model::TxResponse;

/// Wraps a closure that handles a `TxResponse` object.
pub struct WithRawTxResponse<F>(pub F)
where
    F: FnOnce(&TxResponse);

impl<Env, Original, F> RHListItem<Env, Original> for WithRawTxResponse<F>
where
    Env: TxEnv,
    F: FnOnce(&TxResponse),
{
    type Returns = ();
}

impl<Env, Original, F> RHListItemExec<TxResponse, Env, Original> for WithRawTxResponse<F>
where
    Env: TxEnv,
    Original: TopDecodeMulti,
    F: FnOnce(&TxResponse),
{
    fn item_process_result(self, raw_result: &TxResponse) -> Self::Returns {
        (self.0)(raw_result)
    }
}
