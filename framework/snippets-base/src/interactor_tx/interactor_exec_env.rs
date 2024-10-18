use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::types::{
        ManagedAddress, ManagedBuffer, Tx, TxBaseWithEnv, TxEnv, TxEnvWithTxHash, H256,
    },
    scenario_model::TxExpect,
    ScenarioTxEnv, ScenarioTxEnvData,
};
use multiversx_sdk::gateway::GatewayAsyncService;

use crate::InteractorBase;

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub fn tx(&mut self) -> TxBaseWithEnv<InteractorEnvExec<'_, GatewayProxy>> {
        let data = self.new_env_data();
        let env = InteractorEnvExec { world: self, data };
        Tx::new_with_env(env)
    }
}

/// Environment for executing transactions.
pub struct InteractorEnvExec<'w, GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub world: &'w mut InteractorBase<GatewayProxy>,
    pub data: ScenarioTxEnvData,
}

impl<'w, GatewayProxy> TxEnv for InteractorEnvExec<'w, GatewayProxy>
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

impl<'w, GatewayProxy> ScenarioTxEnv for InteractorEnvExec<'w, GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}

impl<'w, GatewayProxy> TxEnvWithTxHash for InteractorEnvExec<'w, GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    fn set_tx_hash(&mut self, tx_hash: H256) {
        self.data.set_tx_hash(tx_hash);
    }

    fn take_tx_hash(&mut self) -> Option<H256> {
        self.data.take_tx_hash()
    }
}
