use std::path::PathBuf;

use multiversx_sc::types::{AnnotatedValue, ManagedAddress, TxBaseWithEnv, TxEnv};

use crate::{api::StaticApi, scenario_model::TxResponse};

pub type TxScenarioBase = TxBaseWithEnv<ScenarioTxEnvironment>;

#[derive(Default, Debug, Clone)]
pub struct ScenarioTxEnvironment {
    pub context_path: PathBuf,
    pub from_annotation: Option<String>,
    pub to_annotation: Option<String>,
    pub response: Option<TxResponse>,
}

impl TxEnv for ScenarioTxEnvironment {
    type Api = StaticApi;

    fn annotate_from<From>(&mut self, to: &From)
    where
        From: AnnotatedValue<ScenarioTxEnvironment, ManagedAddress<StaticApi>>,
    {
        self.from_annotation = Some(to.annotation(self).to_string())
    }

    fn annotate_to<To>(&mut self, to: &To)
    where
        To: AnnotatedValue<ScenarioTxEnvironment, ManagedAddress<StaticApi>>,
    {
        self.to_annotation = Some(to.annotation(self).to_string())
    }

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas(&self) -> u64 {
        // TODO: annotate
        5_000_000
    }
}
