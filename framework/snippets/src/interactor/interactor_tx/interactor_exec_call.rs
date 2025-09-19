use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{
            FunctionCall, RHListExec, Tx, TxBaseWithEnv, TxFromSpecified, TxGas, TxPayment,
            TxToSpecified,
        },
    },
    scenario::tx_to_step::TxToStep,
    scenario_model::{ScCallStep, TxResponse},
    ScenarioTxEnvData,
};
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::{
    interactor::interactor_tx::interactor_prepare_async::InteractorEstimateAsync, InteractorBase,
};

use super::{InteractorEnvExec, InteractorExecStep, InteractorPrepareAsync, InteractorRunAsync};

async fn run_async_call<'w, GatewayProxy, From, To, Payment, Gas, RH>(
    tx: Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        Payment,
        Gas,
        FunctionCall<StaticApi>,
        RH,
    >,
) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    let mut step_wrapper = tx.tx_to_step();
    step_wrapper.env.world.sc_call(&mut step_wrapper.step).await;
    step_wrapper.process_result()
}

async fn estimate_async_call<'w, GatewayProxy, From, To, Payment, Gas, RH>(
    tx: Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        Payment,
        Gas,
        FunctionCall<StaticApi>,
        RH,
    >,
) -> u64
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
{
    let mut step_wrapper = tx.tx_to_step();
    step_wrapper
        .env
        .world
        .sc_estimate(&mut step_wrapper.step)
        .await
}

impl<'w, GatewayProxy, From, To, Payment, Gas, RH> InteractorRunAsync
    for Tx<InteractorEnvExec<'w, GatewayProxy>, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Result = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> impl std::future::Future<Output = Self::Result> {
        run_async_call(self)
    }
}

impl<'w, GatewayProxy, From, To, Payment, Gas, RH> InteractorEstimateAsync
    for Tx<InteractorEnvExec<'w, GatewayProxy>, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
{
    type Result = u64;

    fn estimate(self) -> impl std::future::Future<Output = Self::Result> {
        estimate_async_call(self)
    }
}

impl<'w, GatewayProxy, From, To, Payment, Gas, RH> InteractorPrepareAsync
    for Tx<InteractorEnvExec<'w, GatewayProxy>, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
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

impl<'w, GatewayProxy, RH> InteractorExecStep<'w, GatewayProxy, ScCallStep, RH>
where
    GatewayProxy: GatewayAsyncService,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub async fn run(mut self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        self.step_wrapper
            .env
            .world
            .sc_call(&mut self.step_wrapper.step)
            .await;
        self.step_wrapper.process_result()
    }
}

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
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

        let mut step_wrapper = tx.tx_to_step();
        self.sc_call(&mut step_wrapper.step).await;
        step_wrapper.process_result();

        self
    }
}
