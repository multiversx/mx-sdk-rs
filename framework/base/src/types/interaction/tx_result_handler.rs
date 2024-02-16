use super::TxEnv;

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

/// Indicates that given result handler is empty, i.e. doesn't cause any side effects and returns nothing.
///
/// Implemented for `()` and `OriginalResultMarker`.
pub trait TxEmptyResultHandler<Env>: TxResultHandler<Env>
where
    Env: TxEnv,
{
}

impl<Env> TxEmptyResultHandler<Env> for () where Env: TxEnv {}
