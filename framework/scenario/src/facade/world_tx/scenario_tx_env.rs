use multiversx_chain_scenario_format::interpret_trait::InterpreterContext;
use multiversx_sc::types::{H256, ManagedAddress, ManagedBuffer, TxEnv, TxEnvWithTxHash};

use crate::{ScenarioWorld, api::StaticApi, scenario_model::TxExpect};

/// Designates a tx environment suitable for running scenarios locally.
pub trait ScenarioTxEnv: TxEnv {
    fn env_data(&self) -> &ScenarioTxEnvData;
}

/// The actual data required to run a scenario locally. This is the minimal environment needed to run txs.
#[derive(Default, Debug, Clone)]
pub struct ScenarioTxEnvData {
    pub interpreter_context: InterpreterContext,
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

impl TxEnvWithTxHash for ScenarioTxEnvData {
    fn set_tx_hash(&mut self, tx_hash: H256) {
        assert!(self.tx_hash.is_none(), "tx hash set twice");
        self.tx_hash = Some(tx_hash);
    }

    fn take_tx_hash(&mut self) -> Option<H256> {
        core::mem::take(&mut self.tx_hash)
    }
}

impl ScenarioTxEnvData {
    pub fn interpreter_context(&self) -> InterpreterContext {
        self.interpreter_context.clone()
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
            interpreter_context: InterpreterContext::new()
                .with_dir(self.current_dir.clone())
                .with_allowed_missing_files(),
            tx_hash: None,
        }
    }
}

/// Provides a `run` method for transactions and steps.
pub trait ScenarioTxRun {
    type Returns;

    fn run(self) -> Self::Returns;
}
