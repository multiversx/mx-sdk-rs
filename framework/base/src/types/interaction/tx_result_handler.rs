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
