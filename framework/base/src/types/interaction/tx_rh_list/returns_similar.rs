use core::marker::PhantomData;

use crate::types::TxEnv;

use super::RHListItem;

pub struct ReturnSimilar<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for ReturnSimilar<T> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<T> ReturnSimilar<T> {
    fn new() -> Self {
        Self::default()
    }
}

// impl<Env> TxResultHandler<Env> for ReturnRaw where Env: TxEnv {}

// impl<Env> TxReturn<Env> for ReturnRaw
// where
//     Env: TxEnv,
// {
//     type Returned = ManagedVec<Env::Api, ManagedBuffer<Env::Api>>;

//     fn sync_call_result(
//         self,
//         raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
//     ) -> Self::Returned {
//         raw_results.clone()
//     }
// }
