use multiversx_sc::{
    codec::TopDecodeMulti,
    types::{RHList, RHListItem, ReturnsExact, TxEnv},
};

use crate::scenario_model::{TxResponse, TypedResponse};

use super::ScenarioTxEnvironment;

pub trait RHListItemScenario<Original>: RHListItem<ScenarioTxEnvironment, Original> {
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::Returns;
}

impl<Original> RHListItemScenario<Original> for ReturnsExact
where
    Original: TopDecodeMulti,
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<Original>::from_raw(tx_response);
        response
            .result
            .expect("ReturnsExact expects that transaction is successful")
    }
}
