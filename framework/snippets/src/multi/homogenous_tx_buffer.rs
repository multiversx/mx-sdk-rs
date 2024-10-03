use multiversx_sc_scenario::{
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{RHListExec, TxBaseWithEnv},
    },
    scenario::tx_to_step::{StepWithResponse, StepWrapper, TxToStep},
    scenario_model::TxResponse,
    ScenarioTxEnvData,
};
use multiversx_sdk_http::GatewayHttpProxy;

use crate::{InteractorBase, InteractorEnvExec, InteractorStep, StepBuffer};

pub struct HomogenousTxBuffer<'w, Step, RH> {
    env: InteractorEnvExec<'w>,
    steps: Vec<StepWrapper<ScenarioTxEnvData, Step, RH>>,
}

impl InteractorBase<GatewayHttpProxy> {
    /// Creates a buffer that can hold multiple transactions, and then execute them all at once.
    ///
    /// This buffer holds transactions of the same type (call/deploy) and with identical result handler types.
    /// Therefore, after execution, all results will have the same type.
    pub fn homogenous_call_buffer<Step, RH>(&mut self) -> HomogenousTxBuffer<'_, Step, RH> {
        let data = self.new_env_data();
        let env = InteractorEnvExec { world: self, data };
        HomogenousTxBuffer {
            env,
            steps: Vec::new(),
        }
    }
}

impl<'w, Step, RH> HomogenousTxBuffer<'w, Step, RH>
where
    Step: InteractorStep + StepWithResponse,
    RH: RHListExec<TxResponse, ScenarioTxEnvData>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub fn push_tx<Tx, F>(&mut self, f: F) -> &mut Self
    where
        Tx: TxToStep<ScenarioTxEnvData, RH, Step = Step>,
        F: FnOnce(TxBaseWithEnv<ScenarioTxEnvData>) -> Tx,
    {
        let env = self.env.world.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);

        self.steps.push(tx.tx_to_step());

        self
    }

    pub async fn run(mut self) -> Vec<<RH::ListReturns as NestedTupleFlatten>::Unpacked> {
        let mut step_buffer = StepBuffer::default();
        for step in &mut self.steps {
            step_buffer.refs.push(&mut step.step);
        }
        self.env.world.multi_sc_exec(step_buffer).await;

        self.steps
            .into_iter()
            .map(|step| step.process_result())
            .collect()
    }
}
