use alloc::vec::Vec;
use multiversx_sc::contract_base::CallableContract;

use crate::tx_mock::TxFunctionName;

/// Contains a reference to a contract implementation.
///
/// It can optionally also contain an allowed endpoint whitelist, to simulate multi-contract.
pub struct ContractContainer {
    callable: Box<dyn CallableContract>,
    function_whitelist: Option<Vec<String>>,
    pub panic_message: bool,
}

impl ContractContainer {
    pub fn new(
        callable: Box<dyn CallableContract>,
        function_whitelist: Option<Vec<String>>,
        panic_message: bool,
    ) -> Self {
        ContractContainer {
            callable,
            function_whitelist,
            panic_message,
        }
    }

    fn validate_function_name(&self, function_name: &TxFunctionName) -> bool {
        if let Some(function_whitelist) = &self.function_whitelist {
            function_whitelist
                .iter()
                .any(|whitelisted_endpoint| whitelisted_endpoint.as_str() == function_name.as_str())
        } else {
            true
        }
    }

    pub fn call(&self, function_name: &TxFunctionName) -> bool {
        if self.validate_function_name(function_name) {
            self.callable.call(function_name.as_str())
        } else {
            false
        }
    }
}
