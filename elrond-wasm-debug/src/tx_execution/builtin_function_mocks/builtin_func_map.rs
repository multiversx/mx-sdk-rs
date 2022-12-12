use std::collections::HashMap;

use crate::tx_mock::{TxFunctionName, TxInput};

use super::builtin_func_trait::{BuiltinFunction, BuiltinFunctionEsdtTransferInfo};

pub struct BuiltinFunctionMap {
    func_map: HashMap<String, Box<dyn BuiltinFunction>>,
}

impl BuiltinFunctionMap {
    pub fn init(builtin_funcs: Vec<Box<dyn BuiltinFunction>>) -> Self {
        let mut func_map = HashMap::new();
        for builtin_func in builtin_funcs.into_iter() {
            assert!(
                !func_map.contains_key(builtin_func.name()),
                "duplicate builtin function: {}",
                builtin_func.name()
            );
            func_map.insert(builtin_func.name().to_string(), builtin_func);
        }

        Self { func_map }
    }

    #[allow(clippy::borrowed_box)]
    pub fn get(&self, name: &TxFunctionName) -> Option<&Box<dyn BuiltinFunction>> {
        self.func_map.get(name.as_str())
    }

    pub fn extract_token_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionEsdtTransferInfo {
        if let Some(builtin_func) = self.get(&tx_input.func_name) {
            builtin_func.extract_esdt_transfers(tx_input)
        } else {
            BuiltinFunctionEsdtTransferInfo::empty(tx_input)
        }
    }
}

impl std::fmt::Debug for BuiltinFunctionMap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BuiltinFunctionMap").finish()
    }
}
