use super::*;

use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex, MutexGuard},
};

pub struct ContractMap {
    contract_objs: HashMap<Vec<u8>, ContractContainerRef>,
}

impl fmt::Debug for ContractMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContractMap").finish()
    }
}

impl ContractMap {
    pub fn new() -> Self {
        ContractMap {
            contract_objs: HashMap::new(),
        }
    }

    pub fn get_contract(&self, contract_identifier: &[u8]) -> Option<ContractContainerRef> {
        self.contract_objs.get(contract_identifier).cloned()
    }

    pub fn register_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        contract_container: ContractContainer,
    ) {
        let previous_entry = self.contract_objs.insert(
            contract_bytes,
            ContractContainerRef::new(contract_container),
        );
        assert!(previous_entry.is_none(), "contract inserted twice");
    }

    pub fn contains_contract(&self, contract_bytes: &[u8]) -> bool {
        self.contract_objs.contains_key(contract_bytes)
    }
}

impl Default for ContractMap {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Clone, Debug)]
pub struct ContractMapRef(Arc<Mutex<ContractMap>>);

impl ContractMapRef {
    pub fn new() -> Self {
        ContractMapRef(Arc::new(Mutex::new(ContractMap::new())))
    }

    pub fn lock(&self) -> MutexGuard<'_, ContractMap> {
        self.0.lock().unwrap()
    }
}
