use multiversx_sc_scenario::ScenarioTxEnvData;

use crate::Interactor;

impl Interactor {
    pub(crate) fn new_env_data(&self) -> ScenarioTxEnvData {
        ScenarioTxEnvData {
            context_path: self.current_dir.clone(),
            tx_hash: None,
        }
    }
}

pub trait InteractorPrepareAsync {
    type Exec;

    fn prepare_async(self) -> Self::Exec;
}
