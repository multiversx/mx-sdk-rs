use core::marker::PhantomData;

use crate::types::{ManagedBuffer, ManagedVec, SyncCallRawResult, TxEnv};

use super::{RHListItem, RHListItemExec};

pub struct WithResultRaw<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedVec<Env::Api, ManagedBuffer<Env::Api>>),
{
    _phantom: PhantomData<Env>,
    f: F,
}

impl<Env, F> WithResultRaw<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedVec<Env::Api, ManagedBuffer<Env::Api>>),
{
    pub fn new(f: F) -> Self {
        WithResultRaw {
            _phantom: PhantomData,
            f,
        }
    }
}

impl<Env, F, Original> RHListItem<Env, Original> for WithResultRaw<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedVec<Env::Api, ManagedBuffer<Env::Api>>),
{
    type Returns = ();
}

impl<Env, F, Original> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original>
    for WithResultRaw<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedVec<Env::Api, ManagedBuffer<Env::Api>>),
{
    fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Self::Returns {
        (self.f)(&raw_result.0)
    }
}
