use crate::{
    host::context::{BlockchainUpdate, TxCache, TxInput, TxResult, TxTokenTransfer},
    host::runtime::{RuntimeInstanceCallLambda, RuntimeRef},
    types::VMAddress,
};

pub trait BuiltinFunction {
    /// Function name corresponding the builtin function implementation.
    ///
    /// Currently not used.
    fn name(&self) -> &str;

    /// Extracts data relating ESDT transfers handled by the builtin function, if applicable.
    fn extract_esdt_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionEsdtTransferInfo {
        BuiltinFunctionEsdtTransferInfo::empty(tx_input)
    }

    /// Executes builtin function for the givn `TxInput` and with access to the underlying contracts states via the `TxCache`.
    ///
    /// A few builtin functions (the ones transferring ESDT) can also call the VM after they finish,
    /// so they are given the extra reference to the VM and a lambda closure to execute on the VM
    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        runtime: &RuntimeRef,
        lambda: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: RuntimeInstanceCallLambda;
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
