use multiversx_sc::{
    codec::TopDecodeMulti,
    types::{ManagedAddress, RHList, RHListItem, ReturnsExact, TxEnv, WithResultNewAddress},
};

use crate::{
    api::StaticApi,
    scenario_model::{TxResponse, TypedResponse},
};

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

impl<F, Original> RHListItemScenario<Original> for WithResultNewAddress<ScenarioTxEnvironment, F>
where
    F: FnOnce(&ManagedAddress<StaticApi>),
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::Returns {
        let new_address = tx_response
            .new_deployed_address
            .clone()
            .expect("missing returned address");

        (self.f)(&ManagedAddress::from_address(&new_address));
    }
}
