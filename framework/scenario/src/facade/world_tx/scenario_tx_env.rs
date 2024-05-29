use std::path::PathBuf;

use multiversx_chain_scenario_format::interpret_trait::InterpreterContext;
use multiversx_sc::types::{ManagedAddress, ManagedBuffer, TxEnv, H256};

use crate::{api::StaticApi, scenario_model::TxExpect, ScenarioWorld};

/// Designates a tx environment suitable for running scenarios locally.
pub trait ScenarioTxEnv: TxEnv {
    fn env_data(&self) -> &ScenarioTxEnvData;
}

/// The actual data required to run a scenario locally. This is the minimal environment needed to run txs.
#[derive(Default, Debug, Clone)]
pub struct ScenarioTxEnvData {
    pub context_path: PathBuf,
    pub tx_hash: Option<H256>,
}

impl TxEnv for ScenarioTxEnvData {
    type Api = StaticApi;

    type RHExpect = TxExpect;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas_annotation(&self) -> multiversx_sc::types::ManagedBuffer<Self::Api> {
        ManagedBuffer::from("5,000,000")
    }

    fn default_gas_value(&self) -> u64 {
        5_000_000
    }
}

impl ScenarioTxEnvData {
    pub fn interpreter_context(&self) -> InterpreterContext {
        InterpreterContext::default()
            .with_dir(self.context_path.clone())
            .with_allowed_missing_files()
    }
}

impl ScenarioTxEnv for ScenarioTxEnvData {
    fn env_data(&self) -> &ScenarioTxEnvData {
        self
    }
}

impl ScenarioWorld {
    pub(crate) fn new_env_data(&self) -> ScenarioTxEnvData {
        ScenarioTxEnvData {
            context_path: self.current_dir.clone(),
            tx_hash: None,
        }
    }
}

/// Provides a `run` method for transactions and steps.
pub trait ScenarioTxRun {
    type Returns;

    fn run(self) -> Self::Returns;
}
