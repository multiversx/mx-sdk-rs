use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult, TxTokenTransfer};

pub trait BuiltinFunction {
    fn name(&self) -> &str;

    fn extract_esdt_transfers(&self, _: TxInput) -> Vec<TxTokenTransfer> {
        Vec::new()
    }

    fn execute(&self, tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate);
}
