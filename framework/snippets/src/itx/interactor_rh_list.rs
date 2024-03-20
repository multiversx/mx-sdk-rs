use multiversx_sc_scenario::{
    multiversx_sc::types::{ConsNoRet, ConsRet, OriginalResultMarker, RHList, RHListItem, TxEnv},
    scenario_model::TxResponse,
    RHListItemScenario,
};

pub trait RHListScenario<Env>: RHList<Env>
where
    Env: TxEnv,
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::ListReturns;
}

impl<Env> RHListScenario<Env> for ()
where
    Env: TxEnv,
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::ListReturns {}
}

impl<Env, O> RHListScenario<Env> for OriginalResultMarker<O>
where
    Env: TxEnv,
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::ListReturns {}
}

impl<Env, Head, Tail> RHListScenario<Env> for ConsRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItemScenario<Env, Tail::OriginalResult>,
    Tail: RHListScenario<Env>,
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::ListReturns {
        let head_result = self.head.item_scenario_result(tx_response);
        let tail_result = self.tail.item_scenario_result(tx_response);
        (head_result, tail_result)
    }
}

impl<Env, Head, Tail> RHListScenario<Env> for ConsNoRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItemScenario<Env, Tail::OriginalResult, Returns = ()>,
    Tail: RHListScenario<Env>,
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::ListReturns {
        self.head.item_scenario_result(tx_response);
        self.tail.item_scenario_result(tx_response)
    }
}
