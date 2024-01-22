use multiversx_sc::types::{ConsNoRet, ConsRet, OriginalResultMarker, RHList, RHListItem, TxEnv};

use crate::scenario_model::TxResponse;

use super::{RHListItemScenario, ScenarioTxEnvironment};

pub trait RHListScenario: RHList<ScenarioTxEnvironment> {
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::ListReturns;
}

impl RHListScenario for () {
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::ListReturns {}
}

impl<O> RHListScenario for OriginalResultMarker<O> {
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::ListReturns {}
}

impl<Head, Tail> RHListScenario for ConsRet<ScenarioTxEnvironment, Head, Tail>
where
    Head: RHListItemScenario<Tail::OriginalResult>,
    Tail: RHListScenario,
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::ListReturns {
        let head_result = self.head.item_scenario_result(tx_response);
        let tail_result = self.tail.item_scenario_result(tx_response);
        (head_result, tail_result)
    }
}

impl<Head, Tail> RHListScenario for ConsNoRet<ScenarioTxEnvironment, Head, Tail>
where
    Head: RHListItemScenario<Tail::OriginalResult, Returns = ()>,
    Tail: RHListScenario,
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::ListReturns {
        self.head.item_scenario_result(tx_response);
        self.tail.item_scenario_result(tx_response)
    }
}
