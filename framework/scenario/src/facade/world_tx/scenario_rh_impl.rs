use multiversx_sc::{
    abi::{TypeAbi, TypeAbiFrom},
    codec::TopDecodeMulti,
    tuple_util::NestedTupleFlatten,
    types::{
        ManagedAddress, RHListExec, RHListItemExec, ReturnsHandledOrError,
        ReturnsHandledOrErrorRawResult, ReturnsNewAddress, ReturnsNewManagedAddress,
        ReturnsRawResult, ReturnsResult, ReturnsResultAs, ReturnsResultUnmanaged, TxEnv,
        WithNewAddress, WithResultAs,
    },
};

use crate::{
    imports::TxExpect,
    scenario_model::{CheckValue, TxResponse, TxResponseStatus, TypedResponse},
};

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

impl<Env, Original, T> RHListItemExec<TxResponse, Env, Original> for ReturnsResultAs<T>
where
    Env: TxEnv,
    T: TopDecodeMulti + TypeAbiFrom<Original>,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<T>::from_raw(tx_response);
        response
            .result
            .expect("ReturnsResultAs expects that transaction is successful")
    }
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsResultUnmanaged
where
    Env: TxEnv,
    Original: TypeAbi,
    Original::Unmanaged: TopDecodeMulti,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<Original::Unmanaged>::from_raw(tx_response);
        response
            .result
            .expect("ReturnsResultUnmanaged expects that transaction is successful")
    }
}

impl<Env, Original, T, F> RHListItemExec<TxResponse, Env, Original> for WithResultAs<T, F>
where
    Env: TxEnv,
    T: TopDecodeMulti + TypeAbiFrom<Original>,
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

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsRawResult
where
    Env: TxEnv,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        tx_response.out.clone().into()
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

impl ReturnsHandledOrErrorRawResult for TxResponse {
    type SuccessResult = TxResponse;
    type ErrorResult = TxResponseStatus;
}

impl<Env, Original, NHList> RHListItemExec<TxResponse, Env, Original>
    for ReturnsHandledOrError<Env, Original, TxResponse, NHList>
where
    Env: TxEnv<RHExpect = TxExpect>,
    NHList: RHListExec<TxResponse, Env>,
    NHList::ListReturns: NestedTupleFlatten,
{
    fn item_preprocessing(&self, mut prev: TxExpect) -> TxExpect {
        prev.status = CheckValue::Star;
        prev.message = CheckValue::Star;
        prev
    }

    fn item_process_result(self, raw_result: &TxResponse) -> Self::Returns {
        if raw_result.tx_error.is_success() {
            let tuple_result = self.nested_handlers.list_process_result(raw_result);
            Ok(tuple_result.flatten_unpack())
        } else {
            Err(raw_result.tx_error.clone())
        }
    }
}
