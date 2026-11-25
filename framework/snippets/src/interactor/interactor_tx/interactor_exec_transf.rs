use multiversx_sc_scenario::{
    multiversx_sc::types::{Tx, TxFromSpecified, TxGas, TxPayment, TxToSpecified},
    scenario::tx_to_step::TxToStep,
    scenario_model::TransferStep,
};
use multiversx_sdk::{
    chain_core::types::GasResult, data::transaction::Transaction, gateway::GatewayAsyncService,
};

use crate::{DispatchError, InteractorSimulateGasAsync};

use super::{
    InteractorDispatchAsync, InteractorEnvExec, InteractorExecStep, InteractorIntoSdkTransaction,
    InteractorPrepareAsync, InteractorRunAsync,
};

async fn simulate_gas_async_transfer<'w, GatewayProxy, From, To, Payment, Gas>(
    tx: Tx<InteractorEnvExec<'w, GatewayProxy>, From, To, Payment, Gas, (), ()>,
) -> GasResult
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
{
    let step_wrapper = tx.tx_to_step();
    step_wrapper
        .env
        .world
        .simulate_gas_transfer(step_wrapper.step)
        .await
}

async fn dispatch_async_transfer<'w, GatewayProxy, From, To, Payment, Gas>(
    tx: Tx<InteractorEnvExec<'w, GatewayProxy>, From, To, Payment, Gas, (), ()>,
) -> Result<(), DispatchError>
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
{
    let step_wrapper = tx.tx_to_step();
    step_wrapper
        .env
        .world
        .transfer_dispatch(&step_wrapper.step)
        .await?;
    Ok(())
}

impl<'w, GatewayProxy, From, To, Payment, Gas> InteractorRunAsync
    for Tx<InteractorEnvExec<'w, GatewayProxy>, From, To, Payment, Gas, (), ()>
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
{
    type Result = ();

    async fn run(self) -> Self::Result {
        dispatch_async_transfer(self)
            .await
            .unwrap_or_else(|err| panic!("transfer: {err}"))
    }
}

impl<'w, GatewayProxy, From, To, Payment, Gas> InteractorDispatchAsync
    for Tx<InteractorEnvExec<'w, GatewayProxy>, From, To, Payment, Gas, (), ()>
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
{
    type Result = ();

    fn dispatch(self) -> impl std::future::Future<Output = Result<Self::Result, DispatchError>> {
        dispatch_async_transfer(self)
    }
}

impl<'w, GatewayProxy, From, To, Payment, Gas> InteractorIntoSdkTransaction
    for Tx<InteractorEnvExec<'w, GatewayProxy>, From, To, Payment, Gas, (), ()>
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
{
    fn into_sdk_transaction(self) -> Transaction {
        let step_wrapper = self.tx_to_step();
        step_wrapper
            .env
            .world
            .tx_call_to_blockchain_tx(&step_wrapper.step.tx.to_tx_call())
    }
}

impl<'w, GatewayProxy, From, To, Payment, Gas> InteractorSimulateGasAsync
    for Tx<InteractorEnvExec<'w, GatewayProxy>, From, To, Payment, Gas, (), ()>
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
{
    fn simulate_gas(self) -> impl std::future::Future<Output = GasResult> {
        simulate_gas_async_transfer(self)
    }
}

impl<'w, GatewayProxy, From, To, Payment, Gas> InteractorPrepareAsync
    for Tx<InteractorEnvExec<'w, GatewayProxy>, From, To, Payment, Gas, (), ()>
where
    GatewayProxy: GatewayAsyncService,
    From: TxFromSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    To: TxToSpecified<InteractorEnvExec<'w, GatewayProxy>>,
    Payment: TxPayment<InteractorEnvExec<'w, GatewayProxy>>,
    Gas: TxGas<InteractorEnvExec<'w, GatewayProxy>>,
{
    type Exec = InteractorExecStep<'w, GatewayProxy, TransferStep, ()>;

    fn prepare_async(self) -> Self::Exec {
        InteractorExecStep {
            step_wrapper: self.tx_to_step(),
        }
    }
}

impl<GatewayProxy> InteractorExecStep<'_, GatewayProxy, TransferStep, ()>
where
    GatewayProxy: GatewayAsyncService,
{
    pub async fn run(self) {
        self.step_wrapper
            .env
            .world
            .transfer(self.step_wrapper.step)
            .await;
    }
}
