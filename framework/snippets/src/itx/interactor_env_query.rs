use std::path::PathBuf;

use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{
            AnnotatedValue, FunctionCall, ManagedAddress, Tx, TxBaseWithEnv, TxEnv,
            TxFromSpecified, TxGas, TxPayment, TxToSpecified,
        },
    },
    scenario_env_util::*,
    scenario_model::{ScQueryStep, TxResponse},
    RHListScenario, ScenarioTxEnv, ScenarioTxEnvData, ScenarioTxRun, ScenarioWorld,
};

use crate::{Interactor, InteractorPrepareAsync};

pub struct InteractorEnvQuery<'w> {
    pub world: &'w mut Interactor,
    pub data: ScenarioTxEnvData,
}

impl<'w> TxEnv for InteractorEnvQuery<'w> {
    type Api = StaticApi;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas(&self) -> u64 {
        self.data.default_gas()
    }
}

impl<'w> ScenarioTxEnv for InteractorEnvQuery<'w> {
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}

pub struct InteractorQueryStep<'w, RH>
where
    RH: RHListScenario<InteractorEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    world: &'w mut Interactor,
    sc_query_step: ScQueryStep,
    result_handler: RH,
}

impl<'w, To, RH> InteractorPrepareAsync
    for Tx<InteractorEnvQuery<'w>, (), To, (), (), FunctionCall<StaticApi>, RH>
where
    To: TxToSpecified<InteractorEnvQuery<'w>>,
    RH: RHListScenario<InteractorEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Exec = InteractorQueryStep<'w, RH>;

    fn prepare_async(self) -> Self::Exec {
        let mut sc_query_step = tx_to_sc_query_step(&self.env, self.to, self.data);
        InteractorQueryStep {
            world: self.env.world,
            sc_query_step,
            result_handler: self.result_handler,
        }
    }
}

impl<'w, RH> InteractorQueryStep<'w, RH>
where
    RH: RHListScenario<InteractorEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub async fn run(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let mut sc_call_step = self.sc_query_step;
        self.world.sc_query(&mut sc_call_step).await;
        process_result(sc_call_step.response, self.result_handler)
    }
}

impl Interactor {
    pub fn query(&mut self) -> TxBaseWithEnv<InteractorEnvQuery<'_>> {
        let data = self.new_env_data();
        let env = InteractorEnvQuery { world: self, data };
        Tx::new_with_env(env)
    }

    pub async fn chain_query<To, RH, F>(&mut self, f: F) -> &mut Self
    where
        To: TxToSpecified<ScenarioTxEnvData>,
        RH: RHListScenario<ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        ) -> Tx<ScenarioTxEnvData, (), To, (), (), FunctionCall<StaticApi>, RH>,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);
        let mut step = tx_to_sc_query_step(&tx.env, tx.to, tx.data);
        self.sc_query(&mut step).await;
        process_result(step.response, tx.result_handler);
        self
    }
}
