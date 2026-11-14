use multiversx_sc_scenario::{
    ScenarioTxEnvData,
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
};
use multiversx_sdk::{
    chain_core::types::GasResult, data::transaction::Transaction, gateway::GatewayAsyncService,
};

use crate::TxDispatchSettings;
use crate::{DispatchError, InteractorBase, interactor::interactor_dispatch::print_tx_results};

use super::{
    InteractorDispatchAsync, InteractorEnvExec, InteractorExecStep, InteractorIntoSdkTransaction,
    InteractorPrepareAsync, InteractorRunAsync, InteractorSimulateGasAsync,
};

async fn dispatch_async_call<'w, GatewayProxy, From, To, Payment, Gas, RH>(
    tx: Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        Payment,
        Gas,
        FunctionCall<StaticApi>,
        RH,
    >,
) -> Result<<RH::ListReturns as NestedTupleFlatten>::Unpacked, DispatchError>
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
    step_wrapper
        .env
        .world
        .sc_call_dispatch(&mut step_wrapper.step)
        .await?;
    Ok(step_wrapper.process_result())
}

async fn simulate_gas_async_call<'w, GatewayProxy, From, To, Payment, Gas, RH>(
    tx: Tx<
        InteractorEnvExec<'w, GatewayProxy>,
        From,
        To,
        Payment,
        Gas,
        FunctionCall<StaticApi>,
        RH,
    >,
) -> GasResult
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
{
    let step_wrapper = tx.tx_to_step();
    step_wrapper
        .env
        .world
        .sc_call_simulate(&step_wrapper.step)
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

    async fn run(self) -> Self::Result {
        dispatch_async_call(self)
            .await
            .unwrap_or_else(|err| panic!("SC call: {err}"))
    }
}

impl<'w, GatewayProxy, From, To, Payment, Gas, RH> InteractorDispatchAsync
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

    fn dispatch(self) -> impl std::future::Future<Output = Result<Self::Result, DispatchError>> {
        dispatch_async_call(self)
    }
}

impl<'w, GatewayProxy, From, To, Payment, Gas, RH> InteractorIntoSdkTransaction
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
    fn into_sdk_transaction(self) -> Transaction {
        let step_wrapper = self.tx_to_step();
        step_wrapper
            .env
            .world
            .tx_call_to_blockchain_tx(&step_wrapper.step.tx)
    }

    /// Temporarily replaces the interactor dispatch settings.
    /// A failed tx is reported, but is not an error, the same as for the other tx kinds.
    async fn run_with_settings(self, settings: &TxDispatchSettings) -> anyhow::Result<()> {
        let mut step_wrapper = self.tx_to_step();
        let world = &mut step_wrapper.env.world;
        let previous_settings = std::mem::replace(&mut world.dispatch_settings, settings.clone());
        let result = world.sc_call_dispatch(&mut step_wrapper.step).await;
        world.dispatch_settings = previous_settings;

        match result {
            Ok(()) => {
                print_tx_results(step_wrapper.step.response());
                Ok(())
            }
            Err(DispatchError::Interrupted(_)) => Ok(()),
            Err(DispatchError::TxFailed(status)) => {
                println!("Transaction failed: {status}");
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }
}

/// If a gas limit is set, it acts as an upper bound for the simulation.
impl<'w, GatewayProxy, From, To, Payment, Gas, RH> InteractorSimulateGasAsync
    for Tx<InteractorEnvExec<'w, GatewayProxy>, From, To, Payment, Gas, FunctionCall<StaticApi>, RH>
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
{
    fn simulate_gas(self) -> impl std::future::Future<Output = GasResult> {
        simulate_gas_async_call(self)
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
