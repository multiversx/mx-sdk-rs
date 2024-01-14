use crate::{
    api::ManagedTypeApi,
    formatter::SCLowerHex,
    types::{ManagedBuffer, ManagedBufferCachedBuilder, ManagedVec},
};

use super::{FunctionCall, TxData, TxEnv, TxReturn};

pub trait TxReturnSync<Env>: TxReturn<Env>
where
    Env: TxEnv,
{
    fn sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns;
}

impl<Env> TxReturnSync<Env> for ()
where
    Env: TxEnv,
{
    fn sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns {
    }
}

// impl<Env, Head, Tail> TxReturnSync<Env> for (Head, Tail)
// where
//     Env: TxEnv,
//     Head: TxReturnSync<Env>,
//     Tail: TxReturnSync<Env, Returns = ()>,
// {
//     fn sync_call_result(
//         self,
//         raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
//     ) -> Self::Returns {
//         // tail first
//         self.1.sync_call_result(raw_results);

//         // head at last, which also has a chance of returning
//         self.0.sync_call_result(raw_results)
//     }
// }
