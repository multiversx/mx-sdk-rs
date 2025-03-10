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
/// according to the nested result handlers, or with an error in case of a failed transaction.
pub struct ReturnsHandledOrError<Env, Original, NHList>
where
    Env: TxEnv,
    NHList: RHList<Env>,
{
    _phantom_env: PhantomData<Env>,
    _phantom_original: PhantomData<Original>,
    pub nested_handlers: NHList,
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

impl<Env, Original, NHList> ReturnsHandledOrError<Env, Original, NHList>
where
    Env: TxEnv,
    NHList: RHListExec<TxResponse, Env>,
{
    pub fn returns<RH>(self, item: RH) -> ReturnsHandledOrError<Env, Original, NHList::RetOutput>
    where
        RH: RHListItem<Env, NHList::OriginalResult>,
        NHList: RHListAppendRet<Env, RH>,
    {
        ReturnsHandledOrError {
            _phantom_env: PhantomData,
            _phantom_original: PhantomData,
            nested_handlers: self.nested_handlers.append_ret(item),
        }
    }
}

impl<Env, Original, NHList> RHListItem<Env, Original>
    for ReturnsHandledOrError<Env, Original, NHList>
where
    Env: TxEnv,
    NHList: RHListExec<TxResponse, Env>,
    NHList::ListReturns: NestedTupleFlatten,
{
    type Returns = Result<<NHList::ListReturns as NestedTupleFlatten>::Unpacked, TxResponseStatus>;
}

impl<Env, Original, NHList> RHListItemExec<TxResponse, Env, Original>
    for ReturnsHandledOrError<Env, Original, NHList>
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
