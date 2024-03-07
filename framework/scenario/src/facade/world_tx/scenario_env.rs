use std::path::PathBuf;

use multiversx_sc::types::{AnnotatedValue, ManagedAddress, TxBaseWithEnv, TxEnv};

use crate::{api::StaticApi, scenario_model::TxResponse};

pub type TxScenarioBase = TxBaseWithEnv<ScenarioTxEnvironment>;

#[derive(Default, Debug, Clone)]
pub struct ScenarioTxEnvironment {
    pub context_path: PathBuf,
    pub response: Option<TxResponse>,
}

impl TxEnv for ScenarioTxEnvironment {
    type Api = StaticApi;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas(&self) -> u64 {
        // TODO: annotate
        5_000_000
    }
}
