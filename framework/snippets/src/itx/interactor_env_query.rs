use std::path::PathBuf;

use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{
            AnnotatedValue, FunctionCall, ManagedAddress, ManagedBuffer, RHListExec, Tx,
            TxBaseWithEnv, TxEnv, TxFromSpecified, TxGas, TxPayment, TxToSpecified,
        },
    },
    scenario::tx_to_step::{StepWrapper, TxToQueryStep},
    scenario_model::{ScQueryStep, TxExpect, TxResponse},
    ScenarioTxEnv, ScenarioTxEnvData, ScenarioTxRun, ScenarioWorld,
};

use crate::{Interactor, InteractorPrepareAsync};

pub struct InteractorEnvQuery<'w> {
    pub world: &'w mut Interactor,
    pub data: ScenarioTxEnvData,
}

impl<'w> TxEnv for InteractorEnvQuery<'w> {
    type Api = StaticApi;

    type RHExpect = TxExpect;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas_annotation(&self) -> ManagedBuffer<Self::Api> {
        self.data.default_gas_annotation()
    }

    fn default_gas_value(&self) -> u64 {
        self.data.default_gas_value()
    }
}

impl<'w> ScenarioTxEnv for InteractorEnvQuery<'w> {
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}

pub struct InteractorQueryStep<'w, RH>
where
    RH: RHListExec<TxResponse, InteractorEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    step_wrapper: StepWrapper<InteractorEnvQuery<'w>, ScQueryStep, RH>,
}

impl<'w, To, RH> InteractorPrepareAsync
    for Tx<InteractorEnvQuery<'w>, (), To, (), (), FunctionCall<StaticApi>, RH>
where
    To: TxToSpecified<InteractorEnvQuery<'w>>,
    RH: RHListExec<TxResponse, InteractorEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Exec = InteractorQueryStep<'w, RH>;

    fn prepare_async(self) -> Self::Exec {
        InteractorQueryStep {
            step_wrapper: self.tx_to_query_step(),
        }
    }
}

impl<'w, RH> InteractorQueryStep<'w, RH>
where
    RH: RHListExec<TxResponse, InteractorEnvQuery<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub async fn run(mut self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        self.step_wrapper
            .env
            .world
            .sc_query(&mut self.step_wrapper.step)
            .await;
        self.step_wrapper.process_result()
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
        RH: RHListExec<TxResponse, ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        ) -> Tx<ScenarioTxEnvData, (), To, (), (), FunctionCall<StaticApi>, RH>,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);

        let mut step_wrapper = tx.tx_to_query_step();
        self.sc_query(&mut step_wrapper.step).await;
        step_wrapper.process_result();

        // let mut step = tx_to_sc_query_step(&tx.env, tx.to, tx.data);
        // step.expect = Some(tx.result_handler.list_tx_expect());
        // self.sc_query(&mut step).await;
        // process_result(step.response, tx.result_handler);
        self
    }
}
