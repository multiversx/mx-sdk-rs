use crate::{
    multiversx_chain_vm::{world_mock::ContractContainer, BlockchainMock},
    multiversx_sc::contract_base::{CallableContractBuilder, ContractAbiProvider},
    scenario_format::interpret_trait::InterpreterContext,
};
use std::path::{Path, PathBuf};

/// A facade for contracts tests.
///
/// Contains all the context needed to execute scenarios involving contracts.
///
/// Currently defers most of the operations to the blockchain mock object directly,
/// but that one will be refactored and broken up into smaller pieces.
#[derive(Default, Debug)]
pub struct ScenarioWorld {
    pub blockchain_mock: BlockchainMock,
}

impl ScenarioWorld {
    pub fn new() -> Self {
        ScenarioWorld {
            blockchain_mock: BlockchainMock::new(),
        }
    }

    /// Tells the tests where the crate lies relative to the workspace.
    /// This ensures that the paths are set correctly, including in debug mode.
    pub fn set_current_dir_from_workspace(&mut self, relative_path: &str) -> &mut Self {
        let mut path = find_workspace();
        path.push(relative_path);
        self.blockchain_mock.current_dir = path;
        self
    }

    pub fn current_dir(&self) -> &PathBuf {
        &self.blockchain_mock.current_dir
    }

    pub fn interpreter_context(&self) -> InterpreterContext {
        self.blockchain_mock.interpreter_context()
    }

    pub fn register_contract_container(
        &mut self,
        expression: &str,
        contract_container: ContractContainer,
    ) {
        self.blockchain_mock
            .register_contract_container(expression, contract_container);
    }

    /// Links a contract path in a test to a contract implementation.
    pub fn register_contract<B: CallableContractBuilder>(
        &mut self,
        expression: &str,
        contract_builder: B,
    ) {
        self.blockchain_mock
            .register_contract(expression, contract_builder);
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
        self.blockchain_mock.register_partial_contract::<Abi, B>(
            expression,
            contract_builder,
            sub_contract_name,
        );
    }

    /// Exports current scenario to a JSON file, as created.
    pub fn write_scenario_trace<P: AsRef<Path>>(&mut self, file_path: P) {
        self.blockchain_mock.write_scenario_trace(file_path);
    }

    #[deprecated(
        since = "0.39.0",
        note = "Renamed, use `write_scenario_trace` instead."
    )]
    pub fn write_mandos_trace<P: AsRef<Path>>(&mut self, file_path: P) {
        self.write_scenario_trace(file_path);
    }
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

fn is_target(path_buf: &Path) -> bool {
    path_buf.file_name().unwrap() == "target"
}
