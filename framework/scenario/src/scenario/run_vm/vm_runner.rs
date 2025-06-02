use multiversx_chain_vm::{
    executor_impl::new_experimental_executor,
    host::runtime::{Runtime, RuntimeRef, RuntimeWeakRef},
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

use super::ExecutorConfig;

/// Wraps calls to the blockchain mock,
/// while implementing the StepRunner interface.
#[derive(Default, Debug)]
pub struct ScenarioVMRunner {
    pub contract_map_ref: ContractMapRef,
    pub blockchain_mock: BlockchainMock,
    pub executor_config: ExecutorConfig,
}

impl ScenarioVMRunner {
    pub fn new() -> Self {
        let contract_map_ref = ContractMapRef::new();
        let blockchain_mock = BlockchainMock::default();
        ScenarioVMRunner {
            contract_map_ref,
            blockchain_mock,
            executor_config: ExecutorConfig::default(),
        }
    }

    fn create_executor(
        &self,
        config: &ExecutorConfig,
        weak: RuntimeWeakRef,
    ) -> Box<dyn Executor + Send + Sync> {
        match config {
            ExecutorConfig::Debugger => Box::new(ContractDebugExecutor::new(
                weak,
                self.contract_map_ref.clone(),
            )),
            ExecutorConfig::Custom(f) => f(weak),
            ExecutorConfig::Experimental => new_experimental_executor(weak),
            ExecutorConfig::CompiledFeatureIfElse {
                if_compiled,
                fallback,
            } => {
                if cfg!(feature = "compiled-sc-tests") {
                    self.create_executor(if_compiled, weak)
                } else {
                    self.create_executor(fallback, weak)
                }
            },
            ExecutorConfig::Composite(list) => Box::new(CompositeExecutor::new(
                list.iter()
                    .map(|sub_config| self.create_executor(sub_config, weak.clone()))
                    .collect(),
            )),
        }
    }

    pub fn create_debugger_runtime(&self) -> RuntimeRef {
        RuntimeRef::new_cyclic(|weak| {
            let executor = self.create_executor(&self.executor_config, weak);
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
