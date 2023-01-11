use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult};

use super::builtin_func_trait::{BuiltinFunction, BuiltinFunctionEsdtTransferInfo};

/// Checks that user has appropriate role before calling the builtin function.
pub struct BuiltinFunctionRoleCheckWrapper {
    role_name: &'static str,
    builtin_function: Box<dyn BuiltinFunction>,
}

impl BuiltinFunctionRoleCheckWrapper {
    pub fn new(role_name: &'static str, builtin_function: Box<dyn BuiltinFunction>) -> Self {
        Self {
            role_name,
            builtin_function,
        }
    }
}

impl BuiltinFunction for BuiltinFunctionRoleCheckWrapper {
    fn name(&self) -> &str {
        self.builtin_function.name()
    }

    fn extract_esdt_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionEsdtTransferInfo {
        self.builtin_function.extract_esdt_transfers(tx_input)
    }

    fn execute(&self, tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
        if check_allowed_to_execute(self.role_name, &tx_input, &tx_cache) {
            self.builtin_function.execute(tx_input, tx_cache)
        } else {
            (
                TxResult::from_vm_error("action is not allowed"),
                BlockchainUpdate::empty(),
            )
        }
    }
}

pub fn check_allowed_to_execute(role_name: &str, tx_input: &TxInput, tx_cache: &TxCache) -> bool {
    let token_identifier = tx_input.args[0].clone();
    let available_roles = tx_cache.with_account_mut(&tx_input.to, |account| {
        account.esdt.get_roles(&token_identifier)
    });
    available_roles
        .iter()
        .any(|available_role| available_role.as_slice() == role_name.as_bytes())
}
