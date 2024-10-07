use multiversx_sc_scenario::{
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{
            Code, DeployCall, RHListExec, Tx, TxBaseWithEnv, TxCodeValue, TxFromSpecified, TxGas,
            TxPayment,
        },
    },
    scenario::tx_to_step::TxToStep,
    scenario_model::{ScDeployStep, TxResponse},
    ScenarioTxEnvData,
};
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::InteractorBase;

use super::{InteractorEnvExec, InteractorExecStep, InteractorPrepareAsync};

impl<'w, GatewayProxy, From, Payment, Gas, CodeValue, RH> InteractorPrepareAsync
    for Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        (),
        Payment,
        Gas,
        DeployCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Exec = InteractorExecStep<'w, GatewayProxy, ScDeployStep, RH>;

    fn prepare_async(self) -> Self::Exec {
        InteractorExecStep {
            step_wrapper: self.tx_to_step(),
        }
    }
}

impl<'w, GatewayProxy, RH> InteractorExecStep<'w, GatewayProxy, ScDeployStep, RH>
where
    GatewayProxy: GatewayAsyncService,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
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

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
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
        self.sc_deploy(&mut step_wrapper.step).await;
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
        self.sc_deploy(&mut step_wrapper.step).await;
        step_wrapper.process_result()
    }
}
