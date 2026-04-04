use multiversx_sc_scenario::{ScenarioTxEnvData, imports::InterpreterContext};
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::InteractorBase;

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub(crate) fn new_env_data(&self) -> ScenarioTxEnvData {
        ScenarioTxEnvData {
            interpreter_context: InterpreterContext::new().with_dir(self.current_dir.clone()),
            tx_id: None,
            tx_hash: None,
        }
    }
}

pub trait InteractorPrepareAsync {
    type Exec;

    #[deprecated(
        since = "0.54.0",
        note = "Calling `.prepare_async()` no longer necessary, `.run()` can be called directly."
    )]
    fn prepare_async(self) -> Self::Exec;
}

pub trait InteractorRunAsync {
    type Result;

    fn run(self) -> impl std::future::Future<Output = Self::Result>;
}

pub trait InteractorSimulateGasAsync {
    fn simulate_gas(self) -> impl std::future::Future<Output = u64>;
}
