use std::{ops::Deref, rc::Rc};

use crate::tx_mock::{TxContext, TxInput, TxResult};

use super::{BlockchainUpdate, TxCache};

/// The VM API implementation based on a blockchain mock written in Rust.
/// Implemented as a smart pointer to a TxContext structure, which tracks a blockchain transaction.
#[derive(Debug)]
pub struct TxContextRef(Rc<TxContext>);

pub type DebugApi = TxContextRef;

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
    pub fn new(tx_input: TxInput, tx_cache: TxCache) -> Self {
        Self(Rc::new(TxContext::new(tx_input, tx_cache)))
    }

    pub fn dummy() -> Self {
        Self(Rc::new(TxContext::dummy()))
    }

    pub fn into_blockchain_updates(self) -> BlockchainUpdate {
        let tx_context = Rc::try_unwrap(self.0).unwrap();
        let tx_cache = Rc::try_unwrap(tx_context.tx_cache).unwrap();
        tx_cache.into_blockchain_updates()
    }

    /// Consumes the current API and returns the contained output.
    /// Should be called at the end of a tx execution.
    /// Will fail if any other references to the tx context survive, this must be the last.
    pub fn into_tx_result(self) -> TxResult {
        // TODO: investigate if we can also destroy the Rc
        // can be done if we can make sure that no more references exist at this point
        // let tx_context = Rc::try_unwrap(self.0).unwrap();
        self.tx_result_cell.replace(TxResult::default())
    }
}
