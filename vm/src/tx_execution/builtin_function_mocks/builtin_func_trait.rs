use multiversx_sc::types::Address;

use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult, TxTokenTransfer};

pub trait BuiltinFunction {
    fn name(&self) -> &str;

    fn extract_esdt_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionEsdtTransferInfo {
        BuiltinFunctionEsdtTransferInfo::empty(tx_input)
    }

    fn execute(&self, tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate);
}

/// Contains a builtin function call ESDT transfers (if any) and the real recipient of the transfer
/// (can be different from the "to" field.)
pub struct BuiltinFunctionEsdtTransferInfo {
    pub real_recipient: Address,
    pub transfers: Vec<TxTokenTransfer>,
}

impl BuiltinFunctionEsdtTransferInfo {
    pub fn empty(tx_input: &TxInput) -> Self {
        BuiltinFunctionEsdtTransferInfo {
            real_recipient: tx_input.to.clone(),
            transfers: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.transfers.is_empty()
    }
}
