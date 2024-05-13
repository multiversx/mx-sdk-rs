use crate::types::TxEnv;

/// Result handler list item.
///
/// It acts as a result handler that produces a single result.
pub trait RHListItem<Env, Original>
where
    Env: TxEnv,
{
    type Returns;
}

impl<Env, Original> RHListItem<Env, Original> for ()
where
    Env: TxEnv,
{
    type Returns = ();
}
