use crate::{
    api::ManagedTypeApi,
    formatter::SCLowerHex,
    types::{ManagedBuffer, ManagedBufferCachedBuilder},
};

use super::{FunctionCall, TxEnv};

pub trait TxResultHandler<Env>
where
    Env: TxEnv,
{
    type OriginalResult;
}

impl<Env> TxResultHandler<Env> for ()
where
    Env: TxEnv,
{
    type OriginalResult = ();
}

// impl<Env, Head, Tail> TxResultHandler<Env> for (Head, Tail)
// where
//     Env: TxEnv,
//     Head: TxResultHandler<Env>,
//     Tail: TxResultHandler<Env>,
// {
//     type OriginalResult = Tail::OriginalResult;
// }

pub trait TxRunnableCallback<Env>: TxResultHandler<Env>
where
    Env: TxEnv,
{
    fn run_callback(self, env: &Env);
}

impl<Env> TxRunnableCallback<Env> for ()
where
    Env: TxEnv,
{
    fn run_callback(self, _env: &Env) {}
}
