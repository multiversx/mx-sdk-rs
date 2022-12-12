use std::collections::HashMap;

use crate::tx_mock::TxFunctionName;

use super::builtin_func_trait::BuiltinFunction;

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

    pub fn get(&self, name: &TxFunctionName) -> Option<&Box<dyn BuiltinFunction>> {
        self.func_map.get(name.as_str())
    }
}

impl std::fmt::Debug for BuiltinFunctionMap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BuiltinFunctionMap").finish()
    }
}
