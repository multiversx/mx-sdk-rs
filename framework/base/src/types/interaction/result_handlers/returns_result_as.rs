use core::marker::PhantomData;

use multiversx_sc_codec::TopDecodeMulti;

use crate::{
    abi::TypeAbiFrom,
    contract_base::SyncCallRawResult,
    types::{interaction::decode_result, RHListItem, RHListItemExec, TxEnv},
};

/// Indicates that result will be returned.
///
/// Value will be converted to type `T`, which should be compatible with the original type.
pub struct ReturnsResultAs<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for ReturnsResultAs<T> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<T> ReturnsResultAs<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Env, Original, T> RHListItem<Env, Original> for ReturnsResultAs<T>
where
    Env: TxEnv,
    T: TopDecodeMulti + TypeAbiFrom<Original>,
{
    type Returns = T;
}

impl<Env, Original, T> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original>
    for ReturnsResultAs<T>
where
    Env: TxEnv,
    T: TopDecodeMulti + TypeAbiFrom<Original>,
{
    fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Self::Returns {
        decode_result::<Env::Api, T>(raw_result.0.clone())
    }
}
