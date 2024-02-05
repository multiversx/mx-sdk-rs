use core::marker::PhantomData;

use multiversx_sc::{codec::TopDecodeMulti, types::RHListItem};

use crate::scenario_model::TxResponse;

use super::{RHListItemScenario, ScenarioTxEnvironment};

/// Wraps a closure that handles a `TxResponse` object.
pub struct WithRawTxResponse<F>(pub F)
where
    F: FnOnce(&TxResponse);

impl<F, Original> RHListItem<ScenarioTxEnvironment, Original> for WithRawTxResponse<F>
where
    F: FnOnce(&TxResponse),
{
    type Returns = ();
}

impl<Original, F> RHListItemScenario<Original> for WithRawTxResponse<F>
where
    Original: TopDecodeMulti,
    F: FnOnce(&TxResponse),
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::Returns {
        (self.0)(tx_response)
    }
}
