use crate::types::TxEnv;

/// Result handler list item.
///
/// It acts as a result handler that produces a single result.
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot be used as a decoder result handler (does not implement `RHListItem<{Env}>`)",
    label = "not a valid decoder result handler",
    note = "there are multiple ways to specify the result handling, but `{Self}` is not one of them"
)]
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
