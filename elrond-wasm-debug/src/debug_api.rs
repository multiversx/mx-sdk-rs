use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use elrond_wasm::types::Address;

use crate::{
    tx_mock::{TxCache, TxContext, TxInput, TxManagedTypes, TxOutput, TxResult},
    world_mock::{AccountData, BlockchainMock, BlockchainTxInfo},
};

/// The VM API implementation based on a blockchain mock written in Rust.
/// Implemented as a smart pointer to a TxContext structure, which tracks a blockchain transaction.
#[derive(Clone, Debug)]
pub struct DebugApi(Rc<TxContext>);

impl DebugApi {
    // pub fn new(tx_input: TxInput, blockchain_cell: Rc<RefCell<BlockchainMock>>) -> Self {
    //     DebugApi(Rc::new(TxContext::new(tx_input, blockchain_cell)))
    // }

    pub fn new(tx_context: TxContext) -> Self {
        DebugApi(Rc::new(tx_context))
    }

    pub fn dummy() -> Self {
        DebugApi(Rc::new(TxContext::dummy()))
    }

    // pub fn tx_context_ref(&self) -> &TxContext {
    //     &self.0
    // }

    pub fn input_ref(&self) -> &TxInput {
        self.0.tx_input_box.as_ref()
    }

    pub fn blockchain_cache(&self) -> &TxCache {
        &self.0.blockchain_cache
    }

    pub fn blockchain_ref(&self) -> Ref<BlockchainMock> {
        self.0.blockchain_cache.blockchain_ref()
    }

    pub fn get_account(&self, address: &Address) -> &AccountData {
        self.0.get_account(address)
    }

    pub fn get_contract_account(&self) -> &AccountData {
        self.0.get_contract_account()
    }

    pub fn get_account_mut(&self, address: &Address) -> &mut AccountData {
        self.0.get_account_mut(address)
    }

    pub fn get_contract_account_mut(&self) -> &mut AccountData {
        self.0.get_contract_account_mut()
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

    pub fn result_borrow_mut(&self) -> RefMut<TxResult> {
        self.0.tx_result_cell.borrow_mut()
    }
}
