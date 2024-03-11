use std::path::PathBuf;

use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        AnnotatedValue, Code, DeployCall, FunctionCall, ManagedAddress, ManagedBuffer, RHListSync,
        Tx, TxBaseWithEnv, TxCodeSource, TxCodeSourceSpecified, TxCodeValue, TxEnv,
        TxFromSpecified, TxGas, TxPayment, TxToSpecified,
    },
};

use crate::{
    api::StaticApi,
    scenario_model::{AddressValue, BytesValue, ScCallStep, ScDeployStep, TxResponse},
    ScenarioTxEnv, ScenarioTxRun, ScenarioTxRunOnWorld, ScenarioWorld,
};

use super::{scenario_env_util::*, RHListScenario, ScenarioTxEnvData};

/// Environment for executing transactions.
pub struct ScenarioEnvExec<'w> {
    pub world: &'w mut ScenarioWorld,
    pub data: ScenarioTxEnvData,
}

impl<'w> TxEnv for ScenarioEnvExec<'w> {
    type Api = StaticApi;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas(&self) -> u64 {
        self.data.default_gas()
    }
}

impl<'w> ScenarioTxEnv for ScenarioEnvExec<'w> {
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}

impl<From, To, Payment, Gas, RH> ScenarioTxRunOnWorld
    for Tx<ScenarioTxEnvData, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    From: TxFromSpecified<ScenarioTxEnvData>,
    To: TxToSpecified<ScenarioTxEnvData>,
    Payment: TxPayment<ScenarioTxEnvData>,
    Gas: TxGas<ScenarioTxEnvData>,
    RH: RHListScenario<ScenarioTxEnvData>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run_on_world(self, world: &mut ScenarioWorld) -> Self::Returns {
        let mut step = tx_to_sc_call_step(
            &self.env,
            self.from,
            self.to,
            self.payment,
            self.gas,
            self.data,
        );
        world.sc_call(&mut step);
        process_result(step.response, self.result_handler)
    }
}

impl<'w, From, To, Payment, Gas, RH> ScenarioTxRun
    for Tx<ScenarioEnvExec<'w>, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    From: TxFromSpecified<ScenarioEnvExec<'w>>,
    To: TxToSpecified<ScenarioEnvExec<'w>>,
    Payment: TxPayment<ScenarioEnvExec<'w>>,
    Gas: TxGas<ScenarioEnvExec<'w>>,
    RH: RHListScenario<ScenarioEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> Self::Returns {
        let mut step = tx_to_sc_call_step(
            &self.env,
            self.from,
            self.to,
            self.payment,
            self.gas,
            self.data,
        );
        self.env.world.sc_call(&mut step);
        process_result(step.response, self.result_handler)
    }
}

impl<From, Payment, Gas, CodeValue, RH> ScenarioTxRunOnWorld
    for Tx<
        ScenarioTxEnvData,
        From,
        (),
        Payment,
        Gas,
        DeployCall<ScenarioTxEnvData, Code<CodeValue>>,
        RH,
    >
where
    From: TxFromSpecified<ScenarioTxEnvData>,
    Payment: TxPayment<ScenarioTxEnvData>,
    Gas: TxGas<ScenarioTxEnvData>,
    CodeValue: TxCodeValue<ScenarioTxEnvData>,
    RH: RHListScenario<ScenarioTxEnvData>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run_on_world(self, world: &mut ScenarioWorld) -> Self::Returns {
        let mut step =
            tx_to_sc_deploy_step(&self.env, self.from, self.payment, self.gas, self.data);
        world.sc_deploy(&mut step);
        process_result(step.response, self.result_handler)
    }
}

impl<'w, From, Payment, Gas, CodeValue, RH> ScenarioTxRun
    for Tx<
        ScenarioEnvExec<'w>,
        From,
        (),
        Payment,
        Gas,
        DeployCall<ScenarioEnvExec<'w>, Code<CodeValue>>,
        RH,
    >
where
    From: TxFromSpecified<ScenarioEnvExec<'w>>,
    Payment: TxPayment<ScenarioEnvExec<'w>>,
    Gas: TxGas<ScenarioEnvExec<'w>>,
    CodeValue: TxCodeValue<ScenarioEnvExec<'w>>,
    RH: RHListScenario<ScenarioEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> Self::Returns {
        let mut step =
            tx_to_sc_deploy_step(&self.env, self.from, self.payment, self.gas, self.data);
        self.env.world.sc_deploy(&mut step);
        process_result(step.response, self.result_handler)
    }
}

impl ScenarioWorld {
    fn new_env_data(&self) -> ScenarioTxEnvData {
        ScenarioTxEnvData {
            context_path: self.current_dir.clone(),
            ..Default::default()
        }
    }

    fn wrap_world_ref<'w>(&'w mut self) -> ScenarioEnvExec<'w> {
        let data = self.new_env_data();
        ScenarioEnvExec { world: self, data }
    }

    pub fn tx<'w>(&'w mut self) -> TxBaseWithEnv<ScenarioEnvExec<'w>> {
        Tx::new_with_env(self.wrap_world_ref())
    }

    pub fn chain_tx<STx, F>(&mut self, f: F) -> &mut Self
    where
        STx: ScenarioTxRunOnWorld<Returns = ()>,
        F: FnOnce(TxBaseWithEnv<ScenarioTxEnvData>) -> STx,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);
        tx.run_on_world(self);
        self
    }
}
