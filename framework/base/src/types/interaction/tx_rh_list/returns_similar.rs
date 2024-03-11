use core::marker::PhantomData;

use multiversx_sc_codec::{CodecFrom, TopEncodeMulti};

use crate::types::{
    interaction::contract_call_exec::decode_result, ManagedBuffer, ManagedVec, RHListItemSync,
    TxEnv,
};

use super::RHListItem;

pub struct ReturnsSimilar<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for ReturnsSimilar<T> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<T> ReturnsSimilar<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Env, Original, T> RHListItem<Env, Original> for ReturnsSimilar<T>
where
    Env: TxEnv,
    Original: TopEncodeMulti,
    T: CodecFrom<Original>,
{
    type Returns = T;
}

impl<Env, Original, T> RHListItemSync<Env, Original> for ReturnsSimilar<T>
where
    Env: TxEnv,
    Original: TopEncodeMulti,
    T: CodecFrom<Original>,
{
    fn item_sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns {
        decode_result::<Env::Api, T>(raw_results.clone())
    }
}
