use multiversx_chain_scenario_format::interpret_trait::InterpretableFrom;
use multiversx_chain_vm::world_mock::BlockchainState;

use crate::{
    api::DebugApi,
    debug_executor::ContractContainer,
    multiversx_sc::{
        api,
        contract_base::{CallableContractBuilder, ContractAbiProvider},
    },
    scenario::{run_trace::ScenarioTrace, run_vm::ScenarioVMRunner},
    scenario_format::{interpret_trait::InterpreterContext, value_interpreter::interpret_string},
    scenario_model::BytesValue,
    vm_go_tool::run_vm_go_tool,
};
use std::path::{Path, PathBuf};

use super::debugger_backend::DebuggerBackend;

/// A facade for contracts tests.
///
/// Contains all the context needed to execute scenarios involving contracts.
///
/// Currently defers most of the operations to the blockchain mock object directly,
/// but that one will be refactored and broken up into smaller pieces.
pub struct ScenarioWorld {
    pub(crate) current_dir: PathBuf,
    pub(crate) backend: Backend,
}

pub(crate) enum Backend {
    Debugger(DebuggerBackend),
    VmGoBackend,
}

impl Default for ScenarioWorld {
    fn default() -> Self {
        Self::debugger()
    }
}

impl ScenarioWorld {
    pub fn debugger() -> Self {
        ScenarioWorld {
            current_dir: std::env::current_dir().unwrap(),
            backend: Backend::Debugger(DebuggerBackend {
                vm_runner: ScenarioVMRunner::new(),
                trace: None,
            }),
        }
    }

    /// Backwards compatibility only.
    pub fn new() -> Self {
        Self::debugger()
    }

    pub fn vm_go() -> Self {
        ScenarioWorld {
            current_dir: std::env::current_dir().unwrap(),
            backend: Backend::VmGoBackend,
        }
    }

    /// Runs a scenario file (`.scen.json`) with the configured backend.
    ///
    /// Will crash and produce an output if the test failed for any reason.
    pub fn run<P: AsRef<Path>>(self, relative_path: P) {
        let mut absolute_path = self.current_dir.clone();
        absolute_path.push(relative_path);
        match self.backend {
            Backend::Debugger(mut debugger) => {
                debugger.run_scenario_file(&absolute_path);
            },
            Backend::VmGoBackend => {
                run_vm_go_tool(&absolute_path);
            },
        }
    }

    pub(crate) fn get_debugger_backend(&self) -> &DebuggerBackend {
        if let Backend::Debugger(debugger) = &self.backend {
            debugger
        } else {
            panic!("operation only available for the contract debugger backend")
        }
    }

    pub(crate) fn get_mut_debugger_backend(&mut self) -> &mut DebuggerBackend {
        if let Backend::Debugger(debugger) = &mut self.backend {
            debugger
        } else {
            panic!("operation only available for the contract debugger backend")
        }
    }

    pub(crate) fn get_state(&self) -> &BlockchainState {
        &self.get_debugger_backend().vm_runner.blockchain_mock.state
    }

    pub(crate) fn get_mut_state(&mut self) -> &mut BlockchainState {
        &mut self
            .get_mut_debugger_backend()
            .vm_runner
            .blockchain_mock
            .state
    }

    pub fn start_trace(&mut self) -> &mut Self {
        self.get_mut_debugger_backend().trace = Some(ScenarioTrace::default());
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
        InterpreterContext::default()
            .with_dir(self.current_dir.clone())
            .with_allowed_missing_files()
    }

    /// Convenient way of creating a code expression based on the current context
    /// (i.e. with the paths resolved, as configured).
    pub fn code_expression(&self, path: &str) -> BytesValue {
        BytesValue::interpret_from(path, &self.interpreter_context())
    }

    pub fn register_contract_container(
        &mut self,
        expression: &str,
        contract_container: ContractContainer,
    ) {
        let contract_bytes = interpret_string(expression, &self.interpreter_context());
        self.get_mut_debugger_backend()
            .vm_runner
            .contract_map_ref
            .lock()
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
        if let Some(trace) = &mut self.get_mut_debugger_backend().trace {
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
