use crate::{
    api::ManagedTypeApi,
    formatter::SCLowerHex,
    types::{ManagedBuffer, ManagedBufferCachedBuilder, ManagedVec},
};

use super::{FunctionCall, TxEnv, TxResultHandler};

pub trait TxReturn<Env>: TxResultHandler<Env>
where
    Env: TxEnv,
{
    type Returned;

    fn sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returned;
}

impl<Env> TxReturn<Env> for ()
where
    Env: TxEnv,
{
    type Returned = ();

    fn sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returned {
    }
}

pub struct ReturnRaw;

impl<Env> TxResultHandler<Env> for ReturnRaw where Env: TxEnv {}

impl<Env> TxReturn<Env> for ReturnRaw
where
    Env: TxEnv,
{
    type Returned = ManagedVec<Env::Api, ManagedBuffer<Env::Api>>;

    fn sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returned {
        raw_results.clone()
    }
}
