use multiversx_sc_scenario::{imports::InterpreterContext, ScenarioTxEnvData};

use crate::Interactor;

impl Interactor {
    pub(crate) fn new_env_data(&self) -> ScenarioTxEnvData {
        ScenarioTxEnvData {
            interpreter_context: InterpreterContext::new().with_dir(self.current_dir.clone()),
            tx_hash: None,
        }
    }
}

pub trait InteractorPrepareAsync {
    type Exec;

    fn prepare_async(self) -> Self::Exec;
}
