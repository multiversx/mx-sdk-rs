use multiversx_sc::{
    codec::{CodecFrom, TopDecodeMulti, TopEncodeMulti},
    types::{
        ManagedAddress, RHList, RHListItem, RHListItemExec, ReturnsNewAddress,
        ReturnsNewManagedAddress, ReturnsResult, ReturnsResultConv, TxEnv, WithNewAddress,
        WithResultConv,
    },
};

use crate::{
    api::StaticApi,
    scenario_model::{TxResponse, TypedResponse},
};

use super::ScenarioTxEnvData;

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsResult
where
    Env: TxEnv,
    Original: TopDecodeMulti,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<Original>::from_raw(tx_response);
        response
            .result
            .expect("ReturnsResult expects that transaction is successful")
    }
}

impl<Env, Original, T> RHListItemExec<TxResponse, Env, Original> for ReturnsResultConv<T>
where
    Env: TxEnv,
    Original: TopEncodeMulti,
    T: CodecFrom<Original>,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<T>::from_raw(tx_response);
        response
            .result
            .expect("ReturnsResultConv expects that transaction is successful")
    }
}

impl<Env, Original, T, F> RHListItemExec<TxResponse, Env, Original> for WithResultConv<T, F>
where
    Env: TxEnv,
    Original: TopEncodeMulti,
    T: CodecFrom<Original>,
    F: FnOnce(T),
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<T>::from_raw(tx_response);
        let value = response
            .result
            .expect("ReturnsResult expects that transaction is successful");
        (self.f)(value);
    }
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsNewAddress
where
    Env: TxEnv,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        tx_response
            .new_deployed_address
            .clone()
            .expect("missing returned address")
    }
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsNewManagedAddress
where
    Env: TxEnv,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let new_address = tx_response
            .new_deployed_address
            .clone()
            .expect("missing returned address");

        new_address.into()
    }
}

impl<Env, Original, F> RHListItemExec<TxResponse, Env, Original> for WithNewAddress<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedAddress<Env::Api>),
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let new_address = tx_response
            .new_deployed_address
            .clone()
            .expect("missing returned address");

        (self.f)(&ManagedAddress::from_address(&new_address));
    }
}
