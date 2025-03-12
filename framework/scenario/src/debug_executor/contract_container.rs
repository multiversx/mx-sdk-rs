use multiversx_chain_vm::host::tx_mock::TxFunctionName;
use multiversx_sc::contract_base::CallableContract;
use std::sync::Arc;

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

    /// Dummy object for tests where no proper context is created on stack.
    pub fn dummy() -> Self {
        ContractContainer {
            callable: Box::new(DummyCallableContract),
            function_whitelist: Some(Vec::new()),
            panic_message: true,
        }
    }

    pub fn validate_function_name(&self, function_name: &TxFunctionName) -> bool {
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

#[derive(Clone, Debug)]
pub struct ContractContainerRef(pub(crate) Arc<ContractContainer>);

impl ContractContainerRef {
    pub fn new(contract_container: ContractContainer) -> Self {
        ContractContainerRef(Arc::new(contract_container))
    }

    pub fn has_function(&self, func_name: &str) -> bool {
        let tx_func_name = TxFunctionName::from(func_name);
        self.0.validate_function_name(&tx_func_name)
    }
}

impl core::fmt::Debug for ContractContainer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ContractContainer")
            .field("function_whitelist", &self.function_whitelist)
            .finish()
    }
}

pub struct DummyCallableContract;

impl CallableContract for DummyCallableContract {
    fn call(&self, _fn_name: &str) -> bool {
        false
    }
}
