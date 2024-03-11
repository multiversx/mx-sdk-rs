use std::path::PathBuf;

use multiversx_sc::types::{AnnotatedValue, ManagedAddress, TxBaseWithEnv, TxEnv};

use crate::{
    api::StaticApi, scenario_model::TxResponse, ScenarioTxEnv, ScenarioTxEnvData, ScenarioWorld,
};

pub struct ScenarioEnvQuery<'w> {
    pub world: &'w mut ScenarioWorld,
    pub data: ScenarioTxEnvData,
}

impl<'w> TxEnv for ScenarioEnvQuery<'w> {
    type Api = StaticApi;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas(&self) -> u64 {
        self.data.default_gas()
    }
}

impl<'w> ScenarioTxEnv for ScenarioEnvQuery<'w> {
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}
