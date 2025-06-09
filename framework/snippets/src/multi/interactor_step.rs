use crate::sdk::data::transaction::Transaction;
use multiversx_sc_scenario::{
    mandos_system::ScenarioRunner,
    scenario::tx_to_step::StepWithResponse,
    scenario_model::{AddressValue, ScCallStep, ScDeployStep, TxResponse},
};
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::InteractorBase;

pub enum InteractorStepRef<'a> {
    ScCall(&'a mut ScCallStep),
    ScDeploy(&'a mut ScDeployStep),
}

impl InteractorStepRef<'_> {
    pub fn to_transaction<GatewayProxy: GatewayAsyncService>(
        &self,
        interactor: &InteractorBase<GatewayProxy>,
    ) -> Transaction {
        match self {
            InteractorStepRef::ScCall(sc_call) => interactor.tx_call_to_blockchain_tx(&sc_call.tx),
            InteractorStepRef::ScDeploy(sc_deploy) => {
                interactor.sc_deploy_to_blockchain_tx(sc_deploy)
            },
        }
    }

    pub fn sender_address(&self) -> &AddressValue {
        match self {
            InteractorStepRef::ScCall(sc_call) => &sc_call.tx.from,
            InteractorStepRef::ScDeploy(sc_deploy) => &sc_deploy.tx.from,
        }
    }

    pub fn run_step(&mut self, step_runner: &mut dyn ScenarioRunner) {
        match self {
            InteractorStepRef::ScCall(sc_call) => step_runner.run_sc_call_step(sc_call),
            InteractorStepRef::ScDeploy(sc_deploy) => step_runner.run_sc_deploy_step(sc_deploy),
        }
    }

    pub fn set_response(&mut self, tx_response: TxResponse) {
        match self {
            InteractorStepRef::ScCall(sc_call) => sc_call.save_response(tx_response),
            InteractorStepRef::ScDeploy(sc_deploy) => sc_deploy.save_response(tx_response),
        }
    }
}

/// Describes a scenario step that can be executed in an interactor.
pub trait InteractorStep: StepWithResponse {
    fn as_interactor_step(&mut self) -> InteractorStepRef<'_>;
}

impl InteractorStep for ScCallStep {
    fn as_interactor_step(&mut self) -> InteractorStepRef<'_> {
        InteractorStepRef::ScCall(self)
    }
}

impl InteractorStep for ScDeployStep {
    fn as_interactor_step(&mut self) -> InteractorStepRef<'_> {
        InteractorStepRef::ScDeploy(self)
    }
}
