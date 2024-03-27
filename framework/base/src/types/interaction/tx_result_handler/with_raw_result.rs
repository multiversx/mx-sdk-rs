use core::marker::PhantomData;

use crate::types::{
    ManagedBuffer, ManagedVec, RHListItem, RHListItemExec, SyncCallRawResult, TxEnv,
};

/// Defines a lambda function to be called on the raw result of the transaction.
pub struct WithRawResult<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedVec<Env::Api, ManagedBuffer<Env::Api>>),
{
    _phantom: PhantomData<Env>,
    f: F,
}

impl<Env, F> WithRawResult<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedVec<Env::Api, ManagedBuffer<Env::Api>>),
{
    pub fn new(f: F) -> Self {
        WithRawResult {
            _phantom: PhantomData,
            f,
        }
    }
}

impl<Env, F, Original> RHListItem<Env, Original> for WithRawResult<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedVec<Env::Api, ManagedBuffer<Env::Api>>),
{
    type Returns = ();
}

impl<Env, F, Original> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original>
    for WithRawResult<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedVec<Env::Api, ManagedBuffer<Env::Api>>),
{
    fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Self::Returns {
        (self.f)(&raw_result.0)
    }
}
