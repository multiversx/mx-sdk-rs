use std::path::PathBuf;

use multiversx_sc_scenario::{
    api::StaticApi,
    multiversx_sc::types::{AnnotatedValue, ManagedAddress, TxBaseWithEnv, TxEnv},
    scenario_model::TxResponse,
    ScenarioTxEnvData, ScenarioWorld,
};

use crate::Interactor;

impl Interactor {
    pub(crate) fn new_env_data(&self) -> ScenarioTxEnvData {
        ScenarioTxEnvData {
            context_path: self.current_dir.clone(),
        }
    }
}

pub trait InteractorPrepareAsync {
    type Exec;

    fn prepare_async(self) -> Self::Exec;
}
