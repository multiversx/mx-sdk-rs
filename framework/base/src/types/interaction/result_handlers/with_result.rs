use core::marker::PhantomData;

use multiversx_sc_codec::TopDecodeMulti;

use crate::{
    contract_base::SyncCallRawResult,
    types::{RHListItem, RHListItemExec, TxEnv, interaction::decode_result},
};

/// Defines a lambda function to be called on the decoded result.
///
/// Value will be decoded according to the type defined in the smart contract.
pub struct WithResult<T, F>
where
    F: FnOnce(T),
{
    _phantom: PhantomData<T>,
    f: F,
}

impl<T, F> WithResult<T, F>
where
    F: FnOnce(T),
{
    pub fn new(f: F) -> Self {
        WithResult {
            _phantom: PhantomData,
            f,
        }
    }
}

impl<Env, Original, F> RHListItem<Env, Original> for WithResult<Original, F>
where
    Env: TxEnv,
    F: FnOnce(Original),
{
    type Returns = ();
}

impl<Env, Original, F> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original>
    for WithResult<Original, F>
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
