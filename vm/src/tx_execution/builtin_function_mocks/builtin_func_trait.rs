use crate::{
    tx_execution::{execute_current_tx_context_input, BlockchainVMRef},
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult, TxTokenTransfer},
    types::VMAddress,
};

pub trait BuiltinFunction {
    fn name(&self) -> &str;

    fn extract_esdt_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionEsdtTransferInfo {
        BuiltinFunctionEsdtTransferInfo::empty(tx_input)
    }

    fn execute(
        &self,
        vm: &BlockchainVMRef,
        tx_input: TxInput,
        tx_cache: TxCache,
    ) -> (TxResult, BlockchainUpdate) {
        self.execute_lambda(
            vm,
            tx_input,
            tx_cache,
            Box::new(execute_current_tx_context_input),
        )
    }

    fn execute_lambda<F>(
        &self,
        vm: &BlockchainVMRef,
        tx_input: TxInput,
        tx_cache: TxCache,
        _f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        self.execute(vm, tx_input, tx_cache)
    }
}

/// Contains a builtin function call ESDT transfers (if any) and the real recipient of the transfer
/// (can be different from the "to" field.)
pub struct BuiltinFunctionEsdtTransferInfo {
    pub real_recipient: VMAddress,
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
