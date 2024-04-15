use crate::types::{OriginalResultMarker, TxEnv};

use super::{ConsNoRet, ConsRet, RHList, RHListItem};

/// Indicates how result processing will undergo for one specific result handler.
///
/// Note that the `ResultType` needs to be the first generic type in the definition,
/// so we can add new implementations of the same result handlers for new raw result types in subsequent crates.
pub trait RHListItemExec<RawResult, Env, Original>: RHListItem<Env, Original>
where
    Env: TxEnv,
{
    /// Part of the execution pre-processing, each result handler needs to produce an "expect" field,
    /// as defined in the environment.
    ///
    /// The operation is chained, so all result handlers can contribute, hence the `prev` argument,
    /// which represents the "expect" field produces by the other result handlers.
    ///
    /// The default behavior is to leave it unchanged.
    fn item_tx_expect(&self, prev: Env::RHExpect) -> Env::RHExpect {
        prev
    }

    /// The main functionality of a result handler, it either does some computation internally
    /// (e.g. execution of a lambda function), or produces a result, or both.
    fn item_process_result(self, raw_result: &RawResult) -> Self::Returns;
}

/// Indicates how result processing will undergo for an ensemble of result handlers.
pub trait RHListExec<RawResult, Env>: RHList<Env>
where
    Env: TxEnv,
{
    /// Provides the execution pre-processing, in which result handlers collectively produce an "expect" field.
    ///
    /// The operation starts with the default "expect" field, which normally has all fields unspecified, except
    /// for the "status", which is by default set to "0". This means that failing transactions will cause a panic
    /// unless explicitly stated in one of the result handlers.
    fn list_tx_expect(&self) -> Env::RHExpect;

    /// Aggregates the executions of all result handlers, as configured for a transaction.
    fn list_process_result(self, raw_result: &RawResult) -> Self::ListReturns;
}

impl<RawResult, Env> RHListExec<RawResult, Env> for ()
where
    Env: TxEnv,
{
    fn list_tx_expect(&self) -> Env::RHExpect {
        Env::RHExpect::default()
    }

    fn list_process_result(self, _raw_result: &RawResult) -> Self::ListReturns {}
}

impl<RawResult, Env, O> RHListExec<RawResult, Env> for OriginalResultMarker<O>
where
    Env: TxEnv,
{
    fn list_tx_expect(&self) -> Env::RHExpect {
        Env::RHExpect::default()
    }

    fn list_process_result(self, _raw_result: &RawResult) -> Self::ListReturns {}
}

impl<RawResult, Env, Head, Tail> RHListExec<RawResult, Env> for ConsRet<Env, Head, Tail>
where
    Env: TxEnv,
    Head: RHListItemExec<RawResult, Env, Tail::OriginalResult>,
    Tail: RHListExec<RawResult, Env>,
{
    fn list_tx_expect(&self) -> Env::RHExpect {
        self.head.item_tx_expect(self.tail.list_tx_expect())
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
    fn list_tx_expect(&self) -> Env::RHExpect {
        self.head.item_tx_expect(self.tail.list_tx_expect())
    }

    fn list_process_result(self, raw_result: &RawResult) -> Self::ListReturns {
        self.head.item_process_result(raw_result);
        self.tail.list_process_result(raw_result)
    }
}
