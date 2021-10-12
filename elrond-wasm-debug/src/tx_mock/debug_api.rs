use std::{
    cell::{Ref, RefMut},
    rc::Rc,
};

use crate::world_mock::BlockchainTxInfo;

use super::{TxContext, TxInput, TxManagedTypes, TxOutput};

/// The VM API implementation based on a blockchain mock written in Rust.
/// Implemented as a smart pointer to a TxContext structure, which tracks a blockchain transaction.
#[derive(Clone, Debug)]
pub struct DebugApi(Rc<TxContext>);

impl DebugApi {
    pub fn new(blockchain_info: BlockchainTxInfo, tx_input: TxInput, tx_output: TxOutput) -> Self {
        DebugApi(Rc::new(TxContext::new(
            blockchain_info,
            tx_input,
            tx_output,
        )))
    }

    pub fn dummy() -> Self {
        DebugApi(Rc::new(TxContext::dummy()))
    }

    pub fn input_ref(&self) -> &TxInput {
        self.0.tx_input_box.as_ref()
    }

    pub fn blockchain_info_ref(&self) -> &BlockchainTxInfo {
        self.0.blockchain_info_box.as_ref()
    }

    pub fn m_types_borrow(&self) -> Ref<TxManagedTypes> {
        self.0.managed_types.borrow()
    }

    pub fn m_types_borrow_mut(&self) -> RefMut<TxManagedTypes> {
        self.0.managed_types.borrow_mut()
    }

    /// Consumes the current API and returns the contained output.
    /// Should be called at the end of a tx execution.
    /// Will fail if any other references to the tx context survive, this must be the last.
    pub fn into_output(self) -> TxOutput {
        let tx_context = Rc::try_unwrap(self.0).unwrap();
        tx_context.tx_output_cell.replace(TxOutput::default())
    }

    /// Extracts the output and replaces the one in the TxContext with an empty one.
    pub fn consume_output(&self) -> TxOutput {
        self.0.tx_output_cell.replace(TxOutput::default())
    }

    pub fn output_borrow(&self) -> Ref<TxOutput> {
        self.0.tx_output_cell.borrow()
    }

    pub fn output_borrow_mut(&self) -> RefMut<TxOutput> {
        self.0.tx_output_cell.borrow_mut()
    }
}
