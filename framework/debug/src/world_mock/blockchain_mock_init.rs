use std::path::{Path, PathBuf};

use mandos::{interpret_trait::InterpreterContext, value_interpreter::interpret_string};
use mx_sc::contract_base::{CallableContractBuilder, ContractAbiProvider};

use crate::DebugApi;

use super::{BlockchainMock, ContractContainer};

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
    pub fn interpreter_context(&self) -> InterpreterContext {
        InterpreterContext::new(self.current_dir.clone())
    }

    /// Tells the tests where the crate lies relative to the workspace.
    /// This ensures that the paths are set correctly, including in debug mode.
    pub fn set_current_dir_from_workspace(&mut self, relative_path: &str) -> &mut Self {
        let mut path = find_workspace();
        path.push(relative_path);
        self.current_dir = path;
        self
    }

    pub(crate) fn register_contract_container(
        &mut self,
        expression: &str,
        contract_container: ContractContainer,
    ) {
        let contract_bytes = interpret_string(expression, &self.interpreter_context());
        self.contract_map
            .register_contract(contract_bytes, contract_container);
    }

    /// Links a contract path in a test to a contract implementation.
    pub fn register_contract<B: CallableContractBuilder>(
        &mut self,
        expression: &str,
        contract_builder: B,
    ) {
        self.register_contract_container(
            expression,
            ContractContainer::new(contract_builder.new_contract_obj::<DebugApi>(), None, false),
        )
    }

    #[deprecated(
        since = "0.37.0",
        note = "Got renamed to `register_contract`, but not completely removed, in order to ease test migration. Please replace with `register_contract`."
    )]
    pub fn register_contract_builder<B: CallableContractBuilder>(
        &mut self,
        expression: &str,
        contract_builder: B,
    ) {
        self.register_contract(expression, contract_builder)
    }

    /// Links a contract path in a test to a multi-contract output.
    ///
    /// This simulates the effects of building such a contract with only part of the endpoints.
    pub fn register_partial_contract<Abi, B>(
        &mut self,
        expression: &str,
        contract_builder: B,
        sub_contract_name: &str,
    ) where
        Abi: ContractAbiProvider,
        B: CallableContractBuilder,
    {
        let multi_contract_config = mx_sc_meta::multi_contract_config::<Abi>(
            self.current_dir
                .join("multicontract.toml")
                .to_str()
                .unwrap(),
        );
        let sub_contract = multi_contract_config.find_contract(sub_contract_name);
        let contract_obj = if sub_contract.settings.external_view {
            contract_builder.new_contract_obj::<mx_sc::api::ExternalViewApi<DebugApi>>()
        } else {
            contract_builder.new_contract_obj::<DebugApi>()
        };

        self.register_contract_container(
            expression,
            ContractContainer::new(
                contract_obj,
                Some(sub_contract.all_exported_function_names()),
                sub_contract.settings.panic_message,
            ),
        );
    }
}
