use core::marker::PhantomData;

use crate::types::{ManagedBuffer, ManagedVec, RHListItemSync, TxEnv};

use super::RHListItem;

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

impl<Env, F, Original> RHListItemSync<Env, Original> for WithResultRaw<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedVec<Env::Api, ManagedBuffer<Env::Api>>),
{
    fn item_sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns {
        (self.f)(raw_results)
    }
}
