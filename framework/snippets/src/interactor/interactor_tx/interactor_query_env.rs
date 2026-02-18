use multiversx_sc_scenario::{
    ScenarioTxEnv, ScenarioTxEnvData,
    api::StaticApi,
    imports::{H256, TxEnvWithTxHash, TxId},
    multiversx_sc::types::{ManagedAddress, ManagedBuffer, Tx, TxBaseWithEnv, TxEnv},
    scenario_model::TxExpect,
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

impl<GatewayProxy> TxEnvWithTxHash for InteractorEnvQuery<'_, GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    fn set_tx_id(&mut self, tx_id: TxId) {
        self.data.set_tx_id(tx_id);
    }

    fn take_tx_id(&mut self) -> Option<TxId> {
        self.data.take_tx_id()
    }

    fn set_tx_hash(&mut self, tx_hash: H256) {
        self.data.set_tx_hash(tx_hash);
    }

    fn take_tx_hash(&mut self) -> Option<H256> {
        self.data.take_tx_hash()
    }
}
