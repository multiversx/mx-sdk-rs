use multiversx_sc_scenario::{
    ScenarioTxEnvData,
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{RHListExec, TxBaseWithEnv},
    },
    scenario::tx_to_step::{StepWrapper, TxToStep},
    scenario_model::TxResponse,
};
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::{InteractorBase, InteractorEnvExec, InteractorStep, StepBuffer};

pub struct HomogenousTxBuffer<'w, GatewayProxy, Step, RH>
where
    GatewayProxy: GatewayAsyncService,
{
    env: InteractorEnvExec<'w, GatewayProxy>,
    steps: Vec<StepWrapper<ScenarioTxEnvData, Step, RH>>,
}

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    /// Creates a buffer that can hold multiple transactions, and then execute them all at once.
    ///
    /// This buffer holds transactions of the same type (call/deploy) and with identical result handler types.
    /// Therefore, after execution, all results will have the same type.
    pub fn homogenous_call_buffer<Step, RH>(
        &mut self,
    ) -> HomogenousTxBuffer<'_, GatewayProxy, Step, RH> {
        let data = self.new_env_data();
        let env = InteractorEnvExec { world: self, data };
        HomogenousTxBuffer {
            env,
            steps: Vec::new(),
        }
    }
}

impl<GatewayProxy, Step, RH> HomogenousTxBuffer<'_, GatewayProxy, Step, RH>
where
    GatewayProxy: GatewayAsyncService,
    Step: InteractorStep,
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
            step_buffer.refs.push(step.step.as_interactor_step());
        }
        self.env.world.multi_sc_exec(step_buffer).await;

        self.steps
            .into_iter()
            .map(|step| step.process_result())
            .collect()
    }
}
