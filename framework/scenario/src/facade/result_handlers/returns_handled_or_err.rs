use std::marker::PhantomData;

use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        OriginalResultMarker, RHList, RHListAppendRet, RHListExec, RHListItem, RHListItemExec,
        TxEnv,
    },
};

use crate::scenario_model::{CheckValue, TxExpect, TxResponse, TxResponseStatus};

/// Indicates that a `Result` will be returned, either with the handled result,
/// according to the inner result handlers, or with an error in case of a failed transaction.
pub struct ReturnsHandledOrError<Env, Original, Ok>
where
    Env: TxEnv,
    Ok: RHList<Env>,
{
    _phantom_env: PhantomData<Env>,
    _phantom_original: PhantomData<Original>,
    pub nested_handlers: Ok,
}

impl<Env, Original> Default for ReturnsHandledOrError<Env, Original, OriginalResultMarker<Original>>
where
    Env: TxEnv,
{
    fn default() -> Self {
        ReturnsHandledOrError {
            _phantom_env: PhantomData,
            _phantom_original: PhantomData,
            nested_handlers: OriginalResultMarker::new(),
        }
    }
}

impl<Env, Original> ReturnsHandledOrError<Env, Original, OriginalResultMarker<Original>>
where
    Env: TxEnv,
{
    pub fn new() -> Self {
        ReturnsHandledOrError::default()
    }
}

impl<Env, Original, Ok> ReturnsHandledOrError<Env, Original, Ok>
where
    Env: TxEnv,
    Ok: RHListExec<TxResponse, Env>,
{
    pub fn returns<RH>(self, item: RH) -> ReturnsHandledOrError<Env, Original, Ok::RetOutput>
    where
        RH: RHListItem<Env, Ok::OriginalResult>,
        Ok: RHListAppendRet<Env, RH>,
    {
        ReturnsHandledOrError {
            _phantom_env: PhantomData,
            _phantom_original: PhantomData,
            nested_handlers: self.nested_handlers.append_ret(item),
        }
    }
}

impl<Env, Original, Ok> RHListItem<Env, Original> for ReturnsHandledOrError<Env, Original, Ok>
where
    Env: TxEnv,
    Ok: RHListExec<TxResponse, Env>,
    Ok::ListReturns: NestedTupleFlatten,
{
    type Returns = Result<<Ok::ListReturns as NestedTupleFlatten>::Unpacked, TxResponseStatus>;
}

impl<Env, Original, Ok> RHListItemExec<TxResponse, Env, Original>
    for ReturnsHandledOrError<Env, Original, Ok>
where
    Env: TxEnv<RHExpect = TxExpect>,
    Ok: RHListExec<TxResponse, Env>,
    Ok::ListReturns: NestedTupleFlatten,
{
    fn item_tx_expect(&self, mut prev: TxExpect) -> TxExpect {
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
