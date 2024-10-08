use std::marker::PhantomData;

use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        OriginalResultMarker, RHList, RHListAppendRet, RHListExec, RHListItem, RHListItemExec,
        TxEnv,
    },
};

use crate::scenario_model::{TxResponse, TxResponseStatus};

/// Indicates that a `Result` will be returned, either with the handled result,
/// according to the inner result handlers, or with an error in case of a failed transaction.
pub struct ReturnsResultOrError<Env, Original, Ok>
where
    Env: TxEnv,
    Ok: RHList<Env>,
{
    _phantom_env: PhantomData<Env>,
    _phantom_original: PhantomData<Original>,
    pub ok_t: Ok,
}

impl<Env, Original> Default for ReturnsResultOrError<Env, Original, OriginalResultMarker<Original>>
where
    Env: TxEnv,
{
    fn default() -> Self {
        ReturnsResultOrError {
            _phantom_env: PhantomData,
            _phantom_original: PhantomData,
            ok_t: OriginalResultMarker::new(),
        }
    }
}

impl<Env, Original> ReturnsResultOrError<Env, Original, OriginalResultMarker<Original>>
where
    Env: TxEnv,
{
    pub fn new() -> Self {
        ReturnsResultOrError::default()
    }
}

impl<Env, Original, Ok> ReturnsResultOrError<Env, Original, Ok>
where
    Env: TxEnv,
    Ok: RHListExec<TxResponse, Env>,
{
    pub fn returns<RH>(self, item: RH) -> ReturnsResultOrError<Env, Original, Ok::RetOutput>
    where
        RH: RHListItem<Env, Ok::OriginalResult>,
        Ok: RHListAppendRet<Env, RH>,
    {
        ReturnsResultOrError {
            _phantom_env: PhantomData,
            _phantom_original: PhantomData,
            ok_t: self.ok_t.append_ret(item),
        }
    }
}

impl<Env, Original, Ok> RHListItem<Env, Original> for ReturnsResultOrError<Env, Original, Ok>
where
    Env: TxEnv,
    Ok: RHListExec<TxResponse, Env>,
    Ok::ListReturns: NestedTupleFlatten,
{
    type Returns = Result<<Ok::ListReturns as NestedTupleFlatten>::Unpacked, TxResponseStatus>;
}

impl<Env, Original, Ok> RHListItemExec<TxResponse, Env, Original>
    for ReturnsResultOrError<Env, Original, Ok>
where
    Env: TxEnv,
    Ok: RHListExec<TxResponse, Env>,
    Ok::ListReturns: NestedTupleFlatten,
{
    fn item_process_result(self, raw_result: &TxResponse) -> Self::Returns {
        if raw_result.tx_error.is_success() {
            let tuple_result = self.ok_t.list_process_result(raw_result);
            Ok(tuple_result.flatten_unpack())
        } else {
            Err(raw_result.tx_error.clone())
        }
    }
}
