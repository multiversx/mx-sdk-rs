use multiversx_sc_scenario::{
    imports::{NotPayable, ScCallStep, TxToSpecified, UpgradeCall},
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{
            Code, DeployCall, RHListExec, Tx, TxBaseWithEnv, TxCodeValue, TxFromSpecified, TxGas,
            TxPayment,
        },
    },
    scenario::tx_to_step::{address_annotated, code_annotated, StepWrapper, TxToStep},
    scenario_model::{ScDeployStep, TxResponse},
    ScenarioTxEnvData,
};
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::{InteractorBase, InteractorEstimateAsync};

use super::{InteractorEnvExec, InteractorExecStep, InteractorPrepareAsync, InteractorRunAsync};

#[allow(clippy::type_complexity)]
async fn run_async_upgrade<'w, GatewayProxy, From, To, Gas, CodeValue, RH>(
    tx: Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >,
) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    let mut step_wrapper = tx.tx_to_step();
    step_wrapper.env.world.sc_call(&mut step_wrapper.step).await;
    step_wrapper.process_result()
}

#[allow(clippy::type_complexity)]
async fn estimate_async_upgrade<'w, GatewayProxy, From, To, Gas, CodeValue, RH>(
    tx: Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >,
) where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
{
    let step_wrapper = tx.tx_to_step();
    step_wrapper
        .env
        .world
        .sc_estimate(&step_wrapper.step)
        .await;
}

impl<'w, GatewayProxy, From, To, Gas, CodeValue, RH> InteractorRunAsync
    for Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Result = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> impl std::future::Future<Output = Self::Result> {
        run_async_upgrade(self)
    }
}

impl<'w, GatewayProxy, From, To, Gas, CodeValue, RH> InteractorEstimateAsync
    for Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
{
    type Result = ();

    fn estimate(self) -> impl std::future::Future<Output = Self::Result> {
        estimate_async_upgrade(self)
    }
}

impl<'w, GatewayProxy, From, To, Gas, CodeValue, RH> InteractorPrepareAsync
    for Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        NotPayable,
        Gas,
        UpgradeCall<InteractorEnvExec<'w, GatewayProxy>, Code<CodeValue>>,
        RH,
    >
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    CodeValue: TxCodeValue<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Exec = InteractorExecStep<'w, GatewayProxy, ScCallStep, RH>;

    fn prepare_async(self) -> Self::Exec {
        InteractorExecStep {
            step_wrapper: self.tx_to_step(),
        }
    }
}
