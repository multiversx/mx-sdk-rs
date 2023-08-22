use std::{ops::Deref, sync::Arc};

use crate::tx_mock::{TxContext, TxResult};

use super::{BlockchainUpdate, TxContextStack, TxPanic};

/// The VM API implementation based on a blockchain mock written in Rust.
/// Implemented as a smart pointer to a TxContext structure, which tracks a blockchain transaction.
#[derive(Debug)]
pub struct TxContextRef(Arc<TxContext>);

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

    pub fn new_from_static() -> Self {
        let tx_context_arc = TxContextStack::static_peek();
        Self(tx_context_arc)
    }

    pub fn dummy() -> Self {
        let tx_context = TxContext::dummy();
        let tx_context_arc = Arc::new(tx_context);
        // TODO: WARNING: this does not clean up after itself, must fix!!!
        TxContextStack::static_push(tx_context_arc.clone());
        Self(tx_context_arc)
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
}
