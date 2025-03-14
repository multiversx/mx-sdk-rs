use std::{ops::Deref, sync::Arc};

use crate::host::context::{TxContext, TxResult};

use super::{BlockchainUpdate, TxPanic};

/// The VM API implementation based on a blockchain mock written in Rust.
/// Implemented as a smart pointer to a TxContext structure, which tracks a blockchain transaction.
#[derive(Debug)]
pub struct TxContextRef(pub Arc<TxContext>);

impl Deref for TxContextRef {
    type Target = TxContext;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Clone for TxContextRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl TxContextRef {
    pub fn new(tx_context_arc: Arc<TxContext>) -> Self {
        Self(tx_context_arc)
    }

    pub fn dummy() -> Self {
        Self::new(Arc::new(TxContext::dummy()))
    }

    pub fn into_blockchain_updates(self) -> BlockchainUpdate {
        let tx_context = Arc::try_unwrap(self.0).unwrap();
        let tx_cache = Arc::try_unwrap(tx_context.tx_cache).unwrap();
        tx_cache.into_blockchain_updates()
    }

    /// Consumes the current API and returns the contained output.
    /// Should be called at the end of a tx execution.
    /// Will fail if any other references to the tx context survive, this must be the last.
    pub fn into_tx_result(self) -> TxResult {
        // TODO: investigate if we can also destroy the Arc
        // can be done if we can make sure that no more references exist at this point
        // let tx_context = Arc::try_unwrap(self.0).unwrap();
        std::mem::take(&mut *self.tx_result_cell.lock().unwrap())
    }

    /// The current method for signalling that the current execution is failed, and with what error.
    ///
    /// Note: does not terminate execution or panic, that is handled separately.
    pub fn replace_tx_result_with_error(self, tx_panic: TxPanic) {
        let _ = std::mem::replace(
            &mut *self.tx_result_cell.lock().unwrap(),
            TxResult::from_panic_obj(&tx_panic),
        );
    }

    /// Returns true if the references point to the same `TxContext`.
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        Arc::ptr_eq(&this.0, &other.0)
    }

    pub fn into_ref(self) -> Arc<TxContext> {
        self.0
    }
}
