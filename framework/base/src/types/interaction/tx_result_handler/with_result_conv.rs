use core::marker::PhantomData;

use multiversx_sc_codec::{CodecFrom, TopEncodeMulti};

use crate::types::{
    interaction::decode_result, RHListItem, RHListItemExec, SyncCallRawResult, TxEnv,
};

/// Defines a lambda function to be called on the decoded result.
///
/// Value will be converted to type `T`, which should be compatible with the original type.
pub struct WithResultConv<T, F>
where
    F: FnOnce(T),
{
    _phantom: PhantomData<T>,
    pub f: F,
}

impl<T, F> WithResultConv<T, F>
where
    F: FnOnce(T),
{
    pub fn new(f: F) -> Self {
        WithResultConv {
            _phantom: PhantomData,
            f,
        }
    }
}

impl<Env, Original, T, F> RHListItem<Env, Original> for WithResultConv<T, F>
where
    Env: TxEnv,
    Original: TopEncodeMulti,
    T: CodecFrom<Original>,
    F: FnOnce(T),
{
    type Returns = ();
}

impl<Env, Original, T, F> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original>
    for WithResultConv<T, F>
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
