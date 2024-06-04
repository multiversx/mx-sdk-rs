use multiversx_chain_vm::world_mock::BlockchainState;

use crate::{
    scenario::{run_trace::ScenarioTrace, run_vm::ScenarioVMRunner},
    vm_go_tool::run_mx_scenario_go,
};
use multiversx_sc_meta_lib::tools::find_current_workspace;
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
                run_mx_scenario_go(&absolute_path);
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

    /// Older versions of the Rust compiler were setting a wrong path in the environment when debugging.
    /// This method was made as a workaround to avoid this problem.
    ///
    /// Fortunately, the issue was fixed in Rust, and so this function is no longer necessary.
    #[deprecated(since = "0.50.2", note = "No longer needed, simply delete.")]
    pub fn set_current_dir_from_workspace(&mut self, relative_path: &str) -> &mut Self {
        let mut path = find_current_workspace().unwrap();
        path.push(relative_path);
        self.current_dir = path;
        self
    }

    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
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
