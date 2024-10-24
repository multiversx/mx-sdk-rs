use crate::types::{RHListItem, RHListItemExec, TxEnv};

/// Simply passes a value to the result.
///
/// The value is constant with respect to the tx, will always be returned, irrespective of the tx execution.
///
/// It is especially useful in multi-calls, since it can pass context from the setup section to the result processing section.
pub struct PassValue<T>(pub T);

impl<Env, Original, T> RHListItem<Env, Original> for PassValue<T>
where
    Env: TxEnv,
{
    type Returns = T;
}

impl<RawResult, Env, Original, T> RHListItemExec<RawResult, Env, Original> for PassValue<T>
where
    Env: TxEnv,
{
    fn item_process_result(self, _raw_result: &RawResult) -> T {
        self.0
    }
}
