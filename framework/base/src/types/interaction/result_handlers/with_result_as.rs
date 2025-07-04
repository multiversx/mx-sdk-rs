use core::marker::PhantomData;

use multiversx_sc_codec::TopDecodeMulti;

use crate::{
    abi::TypeAbiFrom,
    contract_base::SyncCallRawResult,
    types::{interaction::decode_result, RHListItem, RHListItemExec, TxEnv},
};

/// Defines a lambda function to be called on the decoded result.
///
/// Value will be converted to type `T`, which should be compatible with the original type.
pub struct WithResultAs<T, F>
where
    F: FnOnce(T),
{
    _phantom: PhantomData<T>,
    pub f: F,
}

impl<T, F> WithResultAs<T, F>
where
    F: FnOnce(T),
{
    pub fn new(f: F) -> Self {
        WithResultAs {
            _phantom: PhantomData,
            f,
        }
    }
}

impl<Env, Original, T, F> RHListItem<Env, Original> for WithResultAs<T, F>
where
    Env: TxEnv,
    T: TopDecodeMulti + TypeAbiFrom<Original>,
    F: FnOnce(T),
{
    type Returns = ();
}

impl<Env, Original, T, F> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original>
    for WithResultAs<T, F>
where
    Env: TxEnv,
    T: TopDecodeMulti + TypeAbiFrom<Original>,
    F: FnOnce(T),
{
    fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Self::Returns {
        let t = decode_result::<Env::Api, T>(raw_result.0.clone());
        (self.f)(t)
    }
}
