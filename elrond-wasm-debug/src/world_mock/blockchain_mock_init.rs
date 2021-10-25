use std::path::{Path, PathBuf};

use elrond_wasm::contract_base::CallableContract;
use mandos::{interpret_trait::InterpreterContext, value_interpreter::interpret_string};

use crate::DebugApi;

use super::BlockchainMock;

fn is_target(path_buf: &Path) -> bool {
    path_buf.file_name().unwrap() == "target"
}

/// Finds the workspace by taking the `current_exe` and working its way up.
/// Works in debug mode too.
pub fn find_workspace() -> PathBuf {
    let current_exe = std::env::current_exe().unwrap();
    let mut path = current_exe.as_path();
    while !is_target(path) {
        path = path.parent().unwrap();
    }

    path.parent().unwrap().into()
}

impl BlockchainMock {
    /// Tells the tests where the crate lies relative to the workspace.
    /// This ensures that the paths are set correctly, including in debug mode.
    pub fn set_current_dir_from_workspace(&mut self, relative_path: &str) {
        let mut path = find_workspace();
        path.push(relative_path);
        self.current_dir = path;
    }

    pub fn register_contract(
        &mut self,
        expression: &str,
        new_contract_closure: Box<dyn Fn(DebugApi) -> Box<dyn CallableContract<DebugApi>>>,
    ) {
        let contract_bytes = interpret_string(
            expression,
            &InterpreterContext::new(self.current_dir.clone()),
        );
        self.contract_map
            .register_contract(contract_bytes, new_contract_closure);
    }
}
