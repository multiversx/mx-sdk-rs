use multiversx_chain_vm::{
    host::runtime::{Runtime, RuntimeRef, RuntimeWeakRef},
    wasmer::{ExperimentalExecutor, WasmerAltExecutor},
};
use multiversx_chain_vm_executor::Executor;

use crate::{
    executor::{
        composite::CompositeExecutor,
        debug::{ContractDebugExecutor, ContractMapRef},
    },
    multiversx_chain_vm::BlockchainMock,
    scenario::{model::*, ScenarioRunner},
};

#[derive(Default, Clone, Copy, Debug)]
pub enum ScenarioExecutorConfig {
    #[default]
    Debugger,
    Wasmer,
    Experimental,
    TryDebuggerThenWasmer,
}

/// Wraps calls to the blockchain mock,
/// while implementing the StepRunner interface.
#[derive(Default, Debug)]
pub struct ScenarioVMRunner {
    pub contract_map_ref: ContractMapRef,
    pub blockchain_mock: BlockchainMock,
    pub executor_config: ScenarioExecutorConfig,
}

impl ScenarioVMRunner {
    pub fn new() -> Self {
        let contract_map_ref = ContractMapRef::new();
        let blockchain_mock = BlockchainMock::default();
        ScenarioVMRunner {
            contract_map_ref,
            blockchain_mock,
            executor_config: ScenarioExecutorConfig::default(),
        }
    }

    fn create_executor(
        &self,
        config: ScenarioExecutorConfig,
        weak: RuntimeWeakRef,
    ) -> Box<dyn Executor + Send + Sync> {
        match config {
            ScenarioExecutorConfig::Debugger => Box::new(ContractDebugExecutor::new(
                weak,
                self.contract_map_ref.clone(),
            )),
            ScenarioExecutorConfig::Wasmer => Box::new(WasmerAltExecutor::new(weak)),
            ScenarioExecutorConfig::Experimental => Box::new(ExperimentalExecutor::new(weak)),
            ScenarioExecutorConfig::TryDebuggerThenWasmer => Box::new(
                CompositeExecutor::new_debugger_then_wasmer(weak, self.contract_map_ref.clone()),
            ),
        }
    }

    pub fn create_debugger_runtime(&self) -> RuntimeRef {
        RuntimeRef::new_cyclic(|weak| {
            let executor = self.create_executor(self.executor_config, weak);
            Runtime::new(self.blockchain_mock.vm.clone(), executor)
        })
    }
}

impl ScenarioRunner for ScenarioVMRunner {
    fn run_external_steps(&mut self, _step: &ExternalStepsStep) {
        panic!("cannot call directly as such")
    }

    fn run_set_state_step(&mut self, step: &SetStateStep) {
        self.perform_set_state(step);
    }

    fn run_sc_call_step(&mut self, step: &mut ScCallStep) {
        self.perform_sc_call_update_results(step);
    }

    fn run_multi_sc_call_step(&mut self, steps: &mut [ScCallStep]) {
        for step in steps {
            self.perform_sc_call_update_results(step);
        }
    }

    fn run_multi_sc_deploy_step(&mut self, steps: &mut [ScDeployStep]) {
        for step in steps.iter_mut() {
            self.perform_sc_deploy_update_results(step);
        }
    }

    fn run_sc_query_step(&mut self, step: &mut ScQueryStep) {
        self.perform_sc_query_update_results(step);
    }

    fn run_sc_deploy_step(&mut self, step: &mut ScDeployStep) {
        self.perform_sc_deploy_update_results(step);
    }

    fn run_transfer_step(&mut self, step: &TransferStep) {
        self.perform_transfer(step);
    }

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep) {
        self.perform_validator_reward(step);
    }

    fn run_check_state_step(&mut self, step: &CheckStateStep) {
        self.perform_check_state(step);
    }

    fn run_dump_state_step(&mut self) {
        self.perform_dump_state();
    }
}
