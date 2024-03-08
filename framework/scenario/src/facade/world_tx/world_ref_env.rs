use std::path::PathBuf;

use multiversx_sc::types::{AnnotatedValue, ManagedAddress, TxBaseWithEnv, TxEnv};

use crate::{api::StaticApi, scenario_model::TxResponse, ScenarioTxEnv, ScenarioWorld};

pub struct WorldRefEnv<'w> {
    pub world: &'w mut ScenarioWorld,
    pub data: ScenarioTxEnv,
}

impl<'w> TxEnv for WorldRefEnv<'w> {
    type Api = StaticApi;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas(&self) -> u64 {
       self.data.default_gas()
    }
}
