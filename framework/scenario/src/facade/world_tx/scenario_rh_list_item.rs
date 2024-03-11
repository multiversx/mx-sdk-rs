use multiversx_sc::{
    codec::{CodecFrom, TopDecodeMulti, TopEncodeMulti},
    types::{
        ManagedAddress, RHList, RHListItem, ReturnsExact, ReturnsSimilar, TxEnv,
        WithResultNewAddress, WithResultSimilar,
    },
};

use crate::{
    api::StaticApi,
    scenario_model::{TxResponse, TypedResponse},
};

use super::ScenarioTxEnvData;

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

impl<Env, Original, T> RHListItemScenario<Env, Original> for ReturnsSimilar<T>
where
    Env: TxEnv,
    Original: TopEncodeMulti,
    T: CodecFrom<Original>,
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<T>::from_raw(tx_response);
        response
            .result
            .expect("ReturnsSimilar expects that transaction is successful")
    }
}

impl<Env, Original, T, F> RHListItemScenario<Env, Original> for WithResultSimilar<T, F>
where
    Env: TxEnv,
    Original: TopEncodeMulti,
    T: CodecFrom<Original>,
    F: FnOnce(T),
{
    fn item_scenario_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<T>::from_raw(tx_response);
        let value = response
            .result
            .expect("ReturnsExact expects that transaction is successful");
        (self.f)(value);
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
