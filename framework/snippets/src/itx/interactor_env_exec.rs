use std::path::PathBuf;

use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{
            AnnotatedValue, Code, DeployCall, FunctionCall, ManagedAddress, ManagedBuffer,
            RHListExec, Tx, TxBaseWithEnv, TxCodeSource, TxCodeSourceSpecified, TxCodeValue, TxEnv,
            TxFromSpecified, TxGas, TxPayment, TxToSpecified,
        },
    },
    scenario_env_util::*,
    scenario_model::{AddressValue, BytesValue, ScCallStep, ScDeployStep, TxExpect, TxResponse},
    ScenarioTxEnv, ScenarioTxEnvData, ScenarioTxRun, ScenarioWorld,
};

use crate::{Interactor, InteractorPrepareAsync};

/// Environment for executing transactions.
pub struct InteractorEnvExec<'w> {
    pub world: &'w mut Interactor,
    pub data: ScenarioTxEnvData,
}

impl<'w> TxEnv for InteractorEnvExec<'w> {
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

impl<'w> ScenarioTxEnv for InteractorEnvExec<'w> {
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}

pub struct InteractorCallStep<'w, RH>
where
    RH: RHListExec<TxResponse, InteractorEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    world: &'w mut Interactor,
    sc_call_step: ScCallStep,
    result_handler: RH,
}

impl<'w, From, To, Payment, Gas, RH> InteractorPrepareAsync
    for Tx<InteractorEnvExec<'w>, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    From: TxFromSpecified<InteractorEnvExec<'w>>,
    To: TxToSpecified<InteractorEnvExec<'w>>,
    Payment: TxPayment<InteractorEnvExec<'w>>,
    Gas: TxGas<InteractorEnvExec<'w>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Exec = InteractorCallStep<'w, RH>;

    fn prepare_async(self) -> Self::Exec {
        let mut sc_call_step = tx_to_sc_call_step(
            &self.env,
            self.from,
            self.to,
            self.payment,
            self.gas,
            self.data,
        );
        InteractorCallStep {
            world: self.env.world,
            sc_call_step,
            result_handler: self.result_handler,
        }
    }
}

impl<'w, RH> InteractorCallStep<'w, RH>
where
    RH: RHListExec<TxResponse, InteractorEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub async fn run(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let mut step = self.sc_call_step;
        step.expect = Some(self.result_handler.list_tx_expect());
        self.world.sc_call(&mut step).await;
        process_result(step.response, self.result_handler)
    }
}

impl Interactor {
    pub fn tx(&mut self) -> TxBaseWithEnv<InteractorEnvExec<'_>> {
        let data = self.new_env_data();
        let env = InteractorEnvExec { world: self, data };
        Tx::new_with_env(env)
    }

    pub async fn chain_call<From, To, Payment, Gas, RH, F>(&mut self, f: F) -> &mut Self
    where
        From: TxFromSpecified<ScenarioTxEnvData>,
        To: TxToSpecified<ScenarioTxEnvData>,
        Payment: TxPayment<ScenarioTxEnvData>,
        Gas: TxGas<ScenarioTxEnvData>,
        RH: RHListExec<TxResponse, ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        )
            -> Tx<ScenarioTxEnvData, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);
        let mut step = tx_to_sc_call_step(&tx.env, tx.from, tx.to, tx.payment, tx.gas, tx.data);
        step.expect = Some(tx.result_handler.list_tx_expect());
        self.sc_call(&mut step).await;
        process_result(step.response, tx.result_handler);
        self
    }
}
