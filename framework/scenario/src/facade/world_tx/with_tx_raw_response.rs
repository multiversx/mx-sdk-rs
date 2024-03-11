use core::marker::PhantomData;

use multiversx_sc::{
    codec::TopDecodeMulti,
    types::{RHListItem, TxEnv},
};

use crate::scenario_model::TxResponse;

use super::{RHListItemScenario, ScenarioTxEnv};

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

impl<Env, Original, F> RHListItemScenario<Env, Original> for WithRawTxResponse<F>
where
    Env: TxEnv,
    Original: TopDecodeMulti,
    F: FnOnce(&TxResponse),
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::Returns {
        (self.0)(tx_response)
    }
}
