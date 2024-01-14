use crate::{
    api::ManagedTypeApi,
    formatter::SCLowerHex,
    types::{ManagedBuffer, ManagedBufferCachedBuilder, ManagedVec},
};

use super::{FunctionCall, ReturnTypeMarker, TxEnv, TxResultHandler};

pub trait TxReturn<Env>: TxResultHandler<Env>
where
    Env: TxEnv,
{
    type Returns;
}

impl<Env> TxReturn<Env> for ()
where
    Env: TxEnv,
{
    type Returns = ();
}

impl<Env, OriginalResult> TxReturn<Env> for ReturnTypeMarker<OriginalResult>
where
    Env: TxEnv,
{
    type Returns = ();
}

// impl<Env, Head, Tail> TxReturn<Env> for (Head, Tail)
// where
//     Env: TxEnv,
//     Head: TxReturn<Env>,
//     Tail: TxReturn<Env, Returns = ()>,
// {
//     type Returns = Head::Returns;
// }
