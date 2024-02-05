use core::marker::PhantomData;

use multiversx_sc_codec::TopDecodeMulti;

use crate::types::{
    interaction::contract_call_exec::decode_result, ManagedBuffer, ManagedVec, RHListItemSync,
    TxEnv,
};

use super::RHListItem;

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

impl<Env, Original, F> RHListItemSync<Env, Original> for WithResultExact<Original, F>
where
    Env: TxEnv,
    Original: TopDecodeMulti,
    F: FnOnce(Original),
{
    fn item_sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns {
        let t = decode_result::<Env::Api, Original>(raw_results.clone());
        (self.f)(t)
    }
}
