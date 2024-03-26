use crate::{proxy_imports::OriginalResultMarker, types::TxEnv};

use super::{ConsNoRet, ConsRet, RHList, RHListItem};

/// Indicates how result processing will undergo for one specific result handler.
///
/// Note that the `ResultType` needs to be the first generic type in the definition,
/// so we can add new implementations of the same result handlers for new raw result types in subsequent crates.
pub trait RHListItemExec<RawResult, Env, Original>: RHListItem<Env, Original>
where
    Env: TxEnv,
{
    fn is_error_handled(&self) -> bool {
        false
    }

    fn item_process_result(self, raw_result: &RawResult) -> Self::Returns;
}

/// Indicates how result processing will undergo for an ensemble of result handlers.
pub trait RHListExec<RawResult, Env>: RHList<Env>
where
    Env: TxEnv,
{
    fn is_error_handled(&self) -> bool;

    fn list_process_result(self, raw_result: &RawResult) -> Self::ListReturns;
}

impl<RawResult, Env> RHListExec<RawResult, Env> for ()
where
    Env: TxEnv,
{
    fn is_error_handled(&self) -> bool {
        false
    }

    fn list_process_result(self, _raw_result: &RawResult) -> Self::ListReturns {}
}

impl<RawResult, Env, O> RHListExec<RawResult, Env> for OriginalResultMarker<O>
where
    Env: TxEnv,
{
    fn is_error_handled(&self) -> bool {
        false
    }

    fn list_process_result(self, _raw_result: &RawResult) -> Self::ListReturns {}
}

impl<RawResult, Env, Head, Tail> RHListExec<RawResult, Env> for ConsRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItemExec<RawResult, Env, Tail::OriginalResult>,
    Tail: RHListExec<RawResult, Env>,
{
    fn is_error_handled(&self) -> bool {
        self.head.is_error_handled() || self.tail.is_error_handled()
    }

    fn list_process_result(self, raw_result: &RawResult) -> Self::ListReturns {
        let head_result = self.head.item_process_result(raw_result);
        let tail_result = self.tail.list_process_result(raw_result);
        (head_result, tail_result)
    }
}

impl<RawResult, Env, Head, Tail> RHListExec<RawResult, Env> for ConsNoRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItemExec<RawResult, Env, Tail::OriginalResult, Returns = ()>,
    Tail: RHListExec<RawResult, Env>,
{
    fn is_error_handled(&self) -> bool {
        self.head.is_error_handled() || self.tail.is_error_handled()
    }

    fn list_process_result(self, raw_result: &RawResult) -> Self::ListReturns {
        self.head.item_process_result(raw_result);
        self.tail.list_process_result(raw_result)
    }
}
