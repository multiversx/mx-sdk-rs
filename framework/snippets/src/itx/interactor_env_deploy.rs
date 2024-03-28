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
    scenario_model::{AddressValue, BytesValue, ScCallStep, ScDeployStep, TxResponse},
    ScenarioEnvExec, ScenarioTxEnv, ScenarioTxEnvData, ScenarioTxRun, ScenarioWorld,
};

use crate::{Interactor, InteractorPrepareAsync};

use super::InteractorEnvExec;

pub struct InteractorDeployStep<'w, RH>
where
    RH: RHListExec<TxResponse, InteractorEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    world: &'w mut Interactor,
    sc_deploy_step: ScDeployStep,
    result_handler: RH,
}

impl<'w, From, Payment, Gas, CodeValue, RH> InteractorPrepareAsync
    for Tx<
        InteractorEnvExec<'w>,
        From,
        (),
        Payment,
        Gas,
        DeployCall<InteractorEnvExec<'w>, Code<CodeValue>>,
        RH,
    >
where
    From: TxFromSpecified<InteractorEnvExec<'w>>,
    Payment: TxPayment<InteractorEnvExec<'w>>,
    Gas: TxGas<InteractorEnvExec<'w>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Exec = InteractorDeployStep<'w, RH>;

    fn prepare_async(self) -> Self::Exec {
        let mut sc_deploy_step =
            tx_to_sc_deploy_step(&self.env, self.from, self.payment, self.gas, self.data);
        InteractorDeployStep {
            world: self.env.world,
            sc_deploy_step,
            result_handler: self.result_handler,
        }
    }
}

impl<'w, RH> InteractorDeployStep<'w, RH>
where
    RH: RHListExec<TxResponse, InteractorEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub async fn run(self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        let mut step = self.sc_deploy_step;
        step.expect = Some(self.result_handler.list_tx_expect());
        self.world.sc_deploy(&mut step).await;
        process_result(step.response, self.result_handler)
    }
}

impl Interactor {
    pub async fn chain_deploy<From, Payment, Gas, CodeValue, RH, F>(&mut self, f: F) -> &mut Self
    where
        From: TxFromSpecified<ScenarioTxEnvData>,
        Payment: TxPayment<ScenarioTxEnvData>,
        Gas: TxGas<ScenarioTxEnvData>,
        CodeValue: TxCodeValue<ScenarioTxEnvData>,
        RH: RHListExec<TxResponse, ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        ) -> Tx<
            ScenarioTxEnvData,
            From,
            (),
            Payment,
            Gas,
            DeployCall<ScenarioTxEnvData, Code<CodeValue>>,
            RH,
        >,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);
        let mut step = tx_to_sc_deploy_step(&tx.env, tx.from, tx.payment, tx.gas, tx.data);
        step.expect = Some(tx.result_handler.list_tx_expect());
        self.sc_deploy(&mut step).await;
        process_result(step.response, tx.result_handler);
        self
    }

    pub async fn run_deploy<From, Payment, Gas, CodeValue, RH, F>(
        &mut self,
        f: F,
    ) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked
    where
        From: TxFromSpecified<ScenarioTxEnvData>,
        Payment: TxPayment<ScenarioTxEnvData>,
        Gas: TxGas<ScenarioTxEnvData>,
        CodeValue: TxCodeValue<ScenarioTxEnvData>,
        RH: RHListExec<TxResponse, ScenarioTxEnvData>,
        RH::ListReturns: NestedTupleFlatten,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        ) -> Tx<
            ScenarioTxEnvData,
            From,
            (),
            Payment,
            Gas,
            DeployCall<ScenarioTxEnvData, Code<CodeValue>>,
            RH,
        >,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);
        let mut step = tx_to_sc_deploy_step(&tx.env, tx.from, tx.payment, tx.gas, tx.data);
        step.expect = Some(tx.result_handler.list_tx_expect());
        self.sc_deploy(&mut step).await;
        process_result(step.response, tx.result_handler)
    }
}
