use alloc::vec::Vec;
use elrond_wasm::contract_base::CallableContract;

/// Contains a reference to a contract implementation.
///
/// It can optionally also contain an allowed endpoint whitelist, to simulate multi-contract.
pub struct ContractContainer {
    callable: Box<dyn CallableContract>,
    endpoint_whitelist: Option<Vec<String>>,
}

impl ContractContainer {
    pub fn new(
        callable: Box<dyn CallableContract>,
        endpoint_whitelist: Option<Vec<String>>,
    ) -> Self {
        ContractContainer {
            callable,
            endpoint_whitelist,
        }
    }

    fn validate_endpoint(&self, endpoint_name: &[u8]) -> bool {
        if let Some(endpoint_filter) = &self.endpoint_whitelist {
            if endpoint_name == &b"init"[..] {
                // init is not in the endpoint list, yet all contract have it
                return true;
            }

            endpoint_filter
                .iter()
                .any(|whitelisted_endpoint| whitelisted_endpoint.as_bytes() == endpoint_name)
        } else {
            true
        }
    }

    pub fn call(&self, endpoint_name: &[u8]) -> bool {
        if self.validate_endpoint(endpoint_name) {
            self.callable.call(endpoint_name)
        } else {
            false
        }
    }
}
