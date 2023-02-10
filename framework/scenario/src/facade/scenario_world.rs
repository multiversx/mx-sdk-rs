use multiversx_chain_vm::DebugApi;

use crate::{
    multiversx_chain_vm::world_mock::ContractContainer,
    multiversx_sc::{
        api,
        contract_base::{CallableContractBuilder, ContractAbiProvider},
    },
    scenario::{run_trace::ScenarioTrace, run_vm::VmAdapter},
    scenario_format::{interpret_trait::InterpreterContext, value_interpreter::interpret_string},
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
    pub current_dir: PathBuf,
    pub vm_runner: VmAdapter, // TODO: convert to option at some point
    pub trace: Option<ScenarioTrace>,
}

impl ScenarioWorld {
    pub fn new() -> Self {
        ScenarioWorld {
            current_dir: std::env::current_dir().unwrap(),
            vm_runner: VmAdapter::new(),
            trace: None,
        }
    }

    pub fn start_trace(&mut self) -> &mut Self {
        self.trace = Some(ScenarioTrace::default());
        self
    }

    /// Tells the tests where the crate lies relative to the workspace.
    /// This ensures that the paths are set correctly, including in debug mode.
    pub fn set_current_dir_from_workspace(&mut self, relative_path: &str) -> &mut Self {
        let mut path = find_workspace();
        path.push(relative_path);
        self.current_dir = path;
        self
    }

    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn interpreter_context(&self) -> InterpreterContext {
        InterpreterContext::new(self.current_dir.clone())
    }

    pub fn register_contract_container(
        &mut self,
        expression: &str,
        contract_container: ContractContainer,
    ) {
        let contract_bytes = interpret_string(expression, &self.interpreter_context());
        self.vm_runner
            .blockchain_mock
            .register_contract_container(contract_bytes, contract_container);
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
        let multi_contract_config = multiversx_sc_meta::multi_contract_config::<Abi>(
            self.current_dir
                .join("multicontract.toml")
                .to_str()
                .unwrap(),
        );
        let sub_contract = multi_contract_config.find_contract(sub_contract_name);
        let contract_obj = if sub_contract.settings.external_view {
            contract_builder.new_contract_obj::<api::ExternalViewApi<DebugApi>>()
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

    /// Exports current scenario to a JSON file, as created.
    pub fn write_scenario_trace<P: AsRef<Path>>(&mut self, file_path: P) {
        if let Some(trace) = &mut self.trace {
            trace.write_scenario_trace(file_path);
        } else {
            panic!("scenario trace no initialized")
        }
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
