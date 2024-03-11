use std::path::PathBuf;

use multiversx_sc::types::{AnnotatedValue, ManagedAddress, TxBaseWithEnv, TxEnv};

use crate::{api::StaticApi, scenario_model::TxResponse};

pub type TxScenarioBase = TxBaseWithEnv<ScenarioTxEnv>;

#[derive(Default, Debug, Clone)]
pub struct ScenarioTxEnv {
    pub context_path: PathBuf,
    pub response: Option<TxResponse>,
}

impl TxEnv for ScenarioTxEnv {
    type Api = StaticApi;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas(&self) -> u64 {
        // TODO: annotate
        5_000_000
    }
}
