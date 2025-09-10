use multiversx_sc_scenario::{imports::InterpreterContext, ScenarioTxEnvData};
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::InteractorBase;

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub(crate) fn new_env_data(&self) -> ScenarioTxEnvData {
        ScenarioTxEnvData {
            interpreter_context: InterpreterContext::new().with_dir(self.current_dir.clone()),
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

pub trait InteractorEstimateAsync {
    type Result;

    fn estimate(self) -> impl std::future::Future<Output = Self::Result>;
}
