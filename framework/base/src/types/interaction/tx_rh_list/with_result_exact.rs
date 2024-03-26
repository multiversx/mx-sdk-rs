use core::marker::PhantomData;

use multiversx_sc_codec::TopDecodeMulti;

use crate::types::{
    interaction::contract_call_exec::decode_result, ManagedBuffer, ManagedVec, SyncCallRawResult,
    TxEnv,
};

use super::{RHListItem, RHListItemExec};

pub struct WithResultExact<T, F>
where
    F: FnOnce(T),
{
    _phantom: PhantomData<T>,
    f: F,
}

impl<T, F> WithResultExact<T, F>
where
    F: FnOnce(T),
{
    pub fn new(f: F) -> Self {
        WithResultExact {
            _phantom: PhantomData,
            f,
        }
    }
}

impl<Env, Original, F> RHListItem<Env, Original> for WithResultExact<Original, F>
where
    Env: TxEnv,
    F: FnOnce(Original),
{
    type Returns = ();
}

impl<Env, Original, F> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original>
    for WithResultExact<Original, F>
where
    Env: TxEnv,
    Original: TopDecodeMulti,
    F: FnOnce(Original),
{
    fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Self::Returns {
        let t = decode_result::<Env::Api, Original>(raw_result.0.clone());
        (self.f)(t)
    }
}
