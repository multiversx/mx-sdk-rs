use std::{
    cell::{Ref, RefCell, RefMut},
    ops::Deref,
    rc::Rc,
};

use elrond_wasm::types::Address;

use crate::{
    tx_mock::{TxCache, TxContext, TxInput, TxManagedTypes, TxOutput, TxResult},
    world_mock::{AccountData, BlockchainMock, BlockchainTxInfo},
};

use super::BlockchainUpdate;

/// The VM API implementation based on a blockchain mock written in Rust.
/// Implemented as a smart pointer to a TxContext structure, which tracks a blockchain transaction.
#[derive(Clone, Debug)]
pub struct TxContextRef(Rc<TxContext>);

pub type DebugApi = TxContextRef;

impl Deref for TxContextRef {
    type Target = TxContext;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl TxContextRef {
    pub fn new(tx_input: TxInput, blockchain_ref: Rc<BlockchainMock>) -> Self {
        Self(Rc::new(TxContext::new(tx_input, blockchain_ref)))
    }

    // pub fn new(tx_context: TxContext) -> Self {
    //     Self(Rc::new(tx_context))
    // }

    pub fn dummy() -> Self {
        Self(Rc::new(TxContext::dummy()))
    }

    pub fn into_blockchain_updates(self) -> BlockchainUpdate {
        let tx_context = Rc::try_unwrap(self.0).unwrap();
        tx_context.blockchain_cache.into_blockchain_updates()
    }

    /// Consumes the current API and returns the contained output.
    /// Should be called at the end of a tx execution.
    /// Will fail if any other references to the tx context survive, this must be the last.
    pub fn into_tx_result(self) -> TxResult {
        let tx_context = Rc::try_unwrap(self.0).unwrap();
        tx_context.tx_result_cell.replace(TxResult::default())
    }

    // /// Extracts the output and replaces the one in the TxContext with an empty one.
    // pub fn consume_output(&self) -> TxOutput {
    //     self.0.tx_output_cell.replace(TxOutput::default())
    // }

    // pub fn output_borrow(&self) -> Ref<TxOutput> {
    //     self.0.tx_output_cell.borrow()
    // }

    // pub fn output_borrow_mut(&self) -> RefMut<TxOutput> {
    //     self.0.tx_output_cell.borrow_mut()
    // }
}
