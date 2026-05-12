use multiversx_sc_scenario::{ScenarioTxEnvData, imports::InterpreterContext};
use multiversx_sdk::{data::transaction::Transaction, gateway::GatewayAsyncService};

use crate::InteractorBase;

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    /// Creates a new [`ScenarioTxEnvData`] initialized with the interactor's current working
    /// directory as the interpreter context. Both `tx_id` and `tx_hash` are left unset and
    /// are expected to be populated later during transaction execution.
    pub(crate) fn new_env_data(&self) -> ScenarioTxEnvData {
        ScenarioTxEnvData {
            interpreter_context: InterpreterContext::new().with_dir(self.current_dir.clone()),
            tx_id: None,
            tx_hash: None,
        }
    }
}

/// Trait for interactor steps that can be prepared for asynchronous execution.
///
/// This trait exists for backwards compatibility only. Since version 0.54.0, calling
/// `.prepare_async()` is no longer required — `.run()` can be invoked directly on the
/// transaction step.
pub trait InteractorPrepareAsync {
    /// The prepared executor type returned by [`prepare_async`](Self::prepare_async).
    type Exec;

    #[deprecated(
        since = "0.54.0",
        note = "Calling `.prepare_async()` no longer necessary, `.run()` can be called directly."
    )]
    /// Prepares the transaction step for asynchronous execution.
    ///
    /// Deprecated since 0.54.0. Use `.run()` directly instead.
    fn prepare_async(self) -> Self::Exec;
}

/// Trait for interactor steps that can be executed asynchronously against a live gateway.
pub trait InteractorRunAsync {
    /// The value produced after the transaction is broadcast and its result is decoded.
    type Result;

    /// Broadcasts the transaction to the network and awaits its result.
    ///
    /// Returns a future that resolves to [`Self::Result`] once the transaction has been
    /// confirmed and the response has been decoded.
    fn run(self) -> impl std::future::Future<Output = Self::Result>;
}

/// Trait for interactor steps that can be converted into a plain SDK [`Transaction`].
///
/// Not implemented for VM queries, which do not produce blockchain transactions.
pub trait InteractorIntoSdkTransaction {
    /// Converts the transaction step into a plain SDK [`Transaction`] without broadcasting it.
    ///
    /// Useful for inspecting or signing the transaction externally before submission.
    fn into_sdk_transaction(self) -> Transaction;
}

/// Trait for interactor steps that support gas estimation via a gateway simulation endpoint.
pub trait InteractorSimulateGasAsync {
    /// Simulates the transaction on the gateway and returns the estimated gas units required.
    ///
    /// The returned future resolves to the gas estimate as a `u64`. No actual state changes
    /// are committed to the network.
    fn simulate_gas(self) -> impl std::future::Future<Output = u64>;
}
