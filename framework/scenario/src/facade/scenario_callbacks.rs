use multiversx_sc::types::{TxCallback, TxRunnableCallback};

use crate::scenario_model::TxResponse;

use super::ScenarioTxEnvironment;

pub struct WithTxResult<F>(pub F)
where
    F: FnOnce(&TxResponse);

impl<F> TxCallback<ScenarioTxEnvironment> for WithTxResult<F> where F: FnOnce(&TxResponse) {}
impl<F> TxRunnableCallback<ScenarioTxEnvironment> for WithTxResult<F>
where
    F: FnOnce(&TxResponse),
{
    fn run_callback(self, env: &ScenarioTxEnvironment) {
        (self.0)(env.response.as_ref().unwrap())
    }
}
