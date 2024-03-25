use core::marker::PhantomData;

use multiversx_sc_codec::{CodecFrom, TopEncodeMulti};

use crate::types::{
    interaction::contract_call_exec::decode_result, ManagedBuffer, ManagedVec, SyncCallRawResult,
    TxEnv,
};

use super::{RHListItem, RHListItemExec};

pub struct WithResultSimilar<T, F>
where
    F: FnOnce(T),
{
    _phantom: PhantomData<T>,
    pub f: F,
}

impl<T, F> WithResultSimilar<T, F>
where
    F: FnOnce(T),
{
    pub fn new(f: F) -> Self {
        WithResultSimilar {
            _phantom: PhantomData,
            f,
        }
    }
}

impl<Env, Original, T, F> RHListItem<Env, Original> for WithResultSimilar<T, F>
where
    Env: TxEnv,
    Original: TopEncodeMulti,
    T: CodecFrom<Original>,
    F: FnOnce(T),
{
    type Returns = ();
}

impl<Env, Original, T, F> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original>
    for WithResultSimilar<T, F>
where
    Env: TxEnv,
    Original: TopEncodeMulti,
    T: CodecFrom<Original>,
    F: FnOnce(T),
{
    fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Self::Returns {
        let t = decode_result::<Env::Api, T>(raw_result.0.clone());
        (self.f)(t)
    }
}
