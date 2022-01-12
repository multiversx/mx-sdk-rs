use std::path::{Path, PathBuf};

use elrond_wasm::{
    api::ExternalViewApi,
    contract_base::{CallableContract, CallableContractBuilder},
};
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

    pub fn register_contract_obj(
        &mut self,
        expression: &str,
        new_contract_obj: Box<dyn CallableContract>,
    ) {
        let contract_bytes = interpret_string(
            expression,
            &InterpreterContext::new(self.current_dir.clone()),
        );
        self.contract_map
            .register_contract(contract_bytes, new_contract_obj);
    }

    pub fn register_contract_builder<B: CallableContractBuilder>(
        &mut self,
        expression: &str,
        contract_builder: B,
    ) {
        self.register_contract_obj(expression, contract_builder.new_contract_obj::<DebugApi>())
    }

    pub fn register_external_view_contract_builder<B: CallableContractBuilder>(
        &mut self,
        expression: &str,
        contract_builder: B,
    ) {
        self.register_contract_obj(
            expression,
            contract_builder.new_contract_obj::<ExternalViewApi<DebugApi>>(),
        )
    }
}
