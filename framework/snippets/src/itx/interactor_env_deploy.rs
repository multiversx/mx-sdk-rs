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
    scenario::tx_to_step::{StepWrapper, TxToStep},
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
    step_wrapper: StepWrapper<InteractorEnvExec<'w>, ScDeployStep, RH>,
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
        InteractorDeployStep {
            step_wrapper: self.tx_to_step(),
        }
    }
}

impl<'w, RH> InteractorDeployStep<'w, RH>
where
    RH: RHListExec<TxResponse, InteractorEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub async fn run(mut self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        self.step_wrapper
            .env
            .world
            .sc_deploy(&mut self.step_wrapper.step)
            .await;
        self.step_wrapper.process_result()
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

        let mut step_wrapper = tx.tx_to_step();
        self.sc_deploy(&mut step_wrapper.step);
        step_wrapper.process_result();

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

        let mut step_wrapper = tx.tx_to_step();
        self.sc_deploy(&mut step_wrapper.step);
        step_wrapper.process_result()
    }
}
