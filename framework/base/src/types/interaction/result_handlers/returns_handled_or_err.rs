use core::marker::PhantomData;

use crate::{
    api::CallTypeApi,
    contract_base::{SyncCallRawResult, SyncCallRawResultOrError},
    tuple_util::NestedTupleFlatten,
    types::{
        OriginalResultMarker, RHList, RHListAppendRet, RHListExec, RHListItem, RHListItemExec,
        TxEnv,
    },
};

/// Defines a type that can be used as raw input for `ReturnsHandledOrError`.
///
/// Allows the raw input type to define the output types.
///
/// Currently implemented for:
/// - SyncCallRawResultOrError - in contracts;
/// - TxResponse - in tests and interactors.
pub trait ReturnsHandledOrErrorRawResult {
    type SuccessResult;
    type ErrorResult;
}

/// Indicates that a `Result` will be returned, either:
/// - with the handled result, according to the nested result handlers,
/// - or with an error in case of a failed transaction.
///
/// Can be used:
/// - in contracts, via `sync_call_fallible`;
/// - in any tests or interactors.
pub struct ReturnsHandledOrError<Env, Original, RawResult, NHList>
where
    Env: TxEnv,
    RawResult: ReturnsHandledOrErrorRawResult,
    NHList: RHList<Env>,
{
    _phantom_env: PhantomData<Env>,
    _phantom_original: PhantomData<Original>,
    _phantom_raw_result: PhantomData<RawResult>,
    pub nested_handlers: NHList,
}

impl<Env, Original, RawResult> Default
    for ReturnsHandledOrError<Env, Original, RawResult, OriginalResultMarker<Original>>
where
    Env: TxEnv,
    RawResult: ReturnsHandledOrErrorRawResult,
{
    fn default() -> Self {
        ReturnsHandledOrError {
            _phantom_env: PhantomData,
            _phantom_original: PhantomData,
            _phantom_raw_result: PhantomData,
            nested_handlers: OriginalResultMarker::new(),
        }
    }
}

impl<Env, Original, RawResult>
    ReturnsHandledOrError<Env, Original, RawResult, OriginalResultMarker<Original>>
where
    Env: TxEnv,
    RawResult: ReturnsHandledOrErrorRawResult,
{
    pub fn new() -> Self {
        ReturnsHandledOrError::default()
    }
}

impl<Env, Original, RawResult, NHList> ReturnsHandledOrError<Env, Original, RawResult, NHList>
where
    Env: TxEnv,
    RawResult: ReturnsHandledOrErrorRawResult,
    NHList: RHListExec<RawResult, Env>,
{
    /// Specifies result handlers for success.
    pub fn returns<RH>(
        self,
        item: RH,
    ) -> ReturnsHandledOrError<Env, Original, RawResult, NHList::RetOutput>
    where
        RH: RHListItem<Env, NHList::OriginalResult>,
        NHList: RHListAppendRet<Env, RH>,
    {
        ReturnsHandledOrError {
            _phantom_env: PhantomData,
            _phantom_original: PhantomData,
            _phantom_raw_result: PhantomData,
            nested_handlers: self.nested_handlers.append_ret(item),
        }
    }
}

impl<Env, Original, RawResult, NHList> RHListItem<Env, Original>
    for ReturnsHandledOrError<Env, Original, RawResult, NHList>
where
    Env: TxEnv,
    RawResult: ReturnsHandledOrErrorRawResult,
    NHList: RHListExec<RawResult::SuccessResult, Env>,
    NHList::ListReturns: NestedTupleFlatten,
{
    type Returns =
        Result<<NHList::ListReturns as NestedTupleFlatten>::Unpacked, RawResult::ErrorResult>;
}

impl<Api> ReturnsHandledOrErrorRawResult for SyncCallRawResultOrError<Api>
where
    Api: CallTypeApi,
{
    type SuccessResult = SyncCallRawResult<Api>;
    type ErrorResult = u32;
}

impl<Env, Original, NHList> RHListItemExec<SyncCallRawResultOrError<Env::Api>, Env, Original>
    for ReturnsHandledOrError<Env, Original, SyncCallRawResultOrError<Env::Api>, NHList>
where
    Env: TxEnv,
    NHList: RHListExec<SyncCallRawResult<Env::Api>, Env>,
    NHList::ListReturns: NestedTupleFlatten,
{
    fn item_process_result(self, raw_result: &SyncCallRawResultOrError<Env::Api>) -> Self::Returns {
        match raw_result {
            SyncCallRawResultOrError::Success(sync_call_raw_result) => {
                let tuple_result = self
                    .nested_handlers
                    .list_process_result(sync_call_raw_result);
                Ok(tuple_result.flatten_unpack())
            }
            SyncCallRawResultOrError::Error(error_code) => Err(*error_code),
        }
    }
}
