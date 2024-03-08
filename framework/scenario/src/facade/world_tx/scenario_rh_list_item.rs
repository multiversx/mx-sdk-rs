use multiversx_sc::{
    codec::TopDecodeMulti,
    types::{ManagedAddress, RHList, RHListItem, ReturnsExact, TxEnv, WithResultNewAddress},
};

use crate::{
    api::StaticApi,
    scenario_model::{TxResponse, TypedResponse},
};

use super::ScenarioTxEnvironment;

pub trait RHListItemScenario<Env, Original>: RHListItem<Env, Original>
where
    Env: TxEnv,
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::Returns;
}

impl<Env, Original> RHListItemScenario<Env, Original> for ReturnsExact
where
    Env: TxEnv,
    Original: TopDecodeMulti,
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<Original>::from_raw(tx_response);
        response
            .result
            .expect("ReturnsExact expects that transaction is successful")
    }
}

impl<Env, Original, F> RHListItemScenario<Env, Original> for WithResultNewAddress<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedAddress<Env::Api>),
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::Returns {
        let new_address = tx_response
            .new_deployed_address
            .clone()
            .expect("missing returned address");

        (self.f)(&ManagedAddress::from_address(&new_address));
    }
}
