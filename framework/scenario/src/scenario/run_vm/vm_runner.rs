use std::{any::Any, cell::RefCell, sync::Arc};

use multiversx_chain_vm::{
    tx_execution::{Runtime, RuntimeRef, RuntimeWeakRef},
    tx_mock::TxFunctionName,
};
use multiversx_chain_vm_executor::Instance;

use crate::{
    debug_executor::{catch_tx_panic, ContractMapRef, DebugSCExecutor, DebugSCInstance},
    multiversx_chain_vm::BlockchainMock,
    scenario::{model::*, ScenarioRunner},
    DebugApi,
};

/// Wraps calls to the blockchain mock,
/// while implementing the StepRunner interface.
#[derive(Default, Debug)]
pub struct ScenarioVMRunner {
    pub contract_map_ref: ContractMapRef,
    pub blockchain_mock: BlockchainMock,
}

impl ScenarioVMRunner {
    pub fn new() -> Self {
        let contract_map_ref = ContractMapRef::new();
        let blockchain_mock = BlockchainMock::new(Box::new(contract_map_ref.clone()));
        ScenarioVMRunner {
            contract_map_ref,
            blockchain_mock,
        }
    }

    pub fn create_debugger_runtime(&self) -> RuntimeRef {
        let runtime_arc = Arc::new_cyclic(|weak| {
            let executor =
                DebugSCExecutor::new(RuntimeWeakRef(weak.clone()), self.contract_map_ref.clone());
            Runtime {
                vm_ref: self.blockchain_mock.vm.clone(),
                override_executor: Some(Box::new(executor)),
                stack: Default::default(),
                current_context_cell: RefCell::new(None),
            }
        });
        RuntimeRef(runtime_arc)
    }

    pub fn wrap_lambda_call<F>(_instance: &dyn Instance, func_name: &str, f: F)
    where
        F: FnOnce(),
    {
        assert_eq!(
            func_name,
            TxFunctionName::WHITEBOX_CALL.as_str(),
            "misconfigured whitebox call"
        );

        // TODO: figure out a way to also validate the instance?

        let result = catch_tx_panic(true, || {
            f();
            Ok(())
        });

        if let Err(tx_panic) = result {
            DebugApi::get_current_tx_context_ref().replace_tx_result_with_error(tx_panic);
        }
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
