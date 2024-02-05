use crate::{
    api::ManagedTypeApi,
    formatter::SCLowerHex,
    types::{ManagedBuffer, ManagedBufferCachedBuilder},
};

use super::{FunctionCall, TxEnv};

pub trait TxCallback<Env>
where
    Env: TxEnv,
{
}

pub trait TxRunnableCallback<Env>: TxCallback<Env>
where
    Env: TxEnv,
{
    fn run_callback(self, env: &Env);
}

impl<Env> TxCallback<Env> for () where Env: TxEnv {}
impl<Env> TxRunnableCallback<Env> for ()
where
    Env: TxEnv,
{
    fn run_callback(self, _env: &Env) {}
}
