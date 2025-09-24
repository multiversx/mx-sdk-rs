use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::{
        tuple_util::NestedTupleFlatten,
        types::{FunctionCall, RHListExec, Tx, TxBaseWithEnv, TxNoPayment, TxToSpecified},
    },
    scenario::tx_to_step::TxToQueryStep,
    scenario_model::TxResponse,
    ScenarioTxEnvData,
};
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::InteractorBase;

use super::{InteractorEnvQuery, InteractorPrepareAsync, InteractorQueryStep, InteractorRunAsync};

async fn run_async_query<'w, GatewayProxy, To, Payment, RH>(
    tx: Tx<InteractorEnvQuery<'w, GatewayProxy>, (), To, Payment, (), FunctionCall<StaticApi>, RH>,
) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked
where
    GatewayProxy: GatewayAsyncService,
    To: TxToSpecified<InteractorEnvQuery<'w, GatewayProxy>>,
    Payment: TxNoPayment<InteractorEnvQuery<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvQuery<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    let mut step_wrapper = tx.tx_to_query_step();
    step_wrapper
        .env
        .world
        .perform_sc_query(&mut step_wrapper.step)
        .await;
    step_wrapper.process_result()
}

impl<'w, GatewayProxy, To, Payment, RH> InteractorRunAsync
    for Tx<InteractorEnvQuery<'w, GatewayProxy>, (), To, Payment, (), FunctionCall<StaticApi>, RH>
where
    GatewayProxy: GatewayAsyncService,
    To: TxToSpecified<InteractorEnvQuery<'w, GatewayProxy>>,
    Payment: TxNoPayment<InteractorEnvQuery<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvQuery<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Result = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> impl std::future::Future<Output = Self::Result> {
        run_async_query(self)
    }
}

impl<'w, GatewayProxy, To, Payment, RH> InteractorPrepareAsync
    for Tx<InteractorEnvQuery<'w, GatewayProxy>, (), To, Payment, (), FunctionCall<StaticApi>, RH>
where
    GatewayProxy: GatewayAsyncService,
    To: TxToSpecified<InteractorEnvQuery<'w, GatewayProxy>>,
    Payment: TxNoPayment<InteractorEnvQuery<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvQuery<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Exec = InteractorQueryStep<'w, GatewayProxy, RH>;

    fn prepare_async(self) -> Self::Exec {
        InteractorQueryStep {
            step_wrapper: self.tx_to_query_step(),
        }
    }
}

impl<'w, GatewayProxy, RH> InteractorQueryStep<'w, GatewayProxy, RH>
where
    GatewayProxy: GatewayAsyncService,
    RH: RHListExec<TxResponse, InteractorEnvQuery<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub async fn run(mut self) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked {
        self.step_wrapper
            .env
            .world
            .perform_sc_query(&mut self.step_wrapper.step)
            .await;
        self.step_wrapper.process_result()
    }
}

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub async fn chain_query<To, Payment, RH, F>(&mut self, f: F) -> &mut Self
    where
        To: TxToSpecified<ScenarioTxEnvData>,
        Payment: TxNoPayment<ScenarioTxEnvData>,
        RH: RHListExec<TxResponse, ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        )
            -> Tx<ScenarioTxEnvData, (), To, Payment, (), FunctionCall<StaticApi>, RH>,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);

        let mut step_wrapper = tx.tx_to_query_step();
        self.perform_sc_query(&mut step_wrapper.step).await;
        step_wrapper.process_result();
        self
    }
}
