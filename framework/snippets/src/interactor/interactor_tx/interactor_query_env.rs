use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::types::{ManagedAddress, ManagedBuffer, Tx, TxBaseWithEnv, TxEnv},
    scenario_model::TxExpect,
    ScenarioTxEnv, ScenarioTxEnvData,
};
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::InteractorBase;

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub fn query(&mut self) -> TxBaseWithEnv<InteractorEnvQuery<'_, GatewayProxy>> {
        let data = self.new_env_data();
        let env = InteractorEnvQuery { world: self, data };
        Tx::new_with_env(env)
    }
}

pub struct InteractorEnvQuery<'w, GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub world: &'w mut InteractorBase<GatewayProxy>,
    pub data: ScenarioTxEnvData,
}

impl<GatewayProxy> TxEnv for InteractorEnvQuery<'_, GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    type Api = StaticApi;

    type RHExpect = TxExpect;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas_annotation(&self) -> ManagedBuffer<Self::Api> {
        self.data.default_gas_annotation()
    }

    fn default_gas_value(&self) -> u64 {
        self.data.default_gas_value()
    }
}

impl<GatewayProxy> ScenarioTxEnv for InteractorEnvQuery<'_, GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}
