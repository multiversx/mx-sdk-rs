use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc::types::{AnnotatedValue, ManagedBuffer, TimestampMillis, TimestampSeconds};

use crate::{
    ScenarioTxEnvData,
    imports::StaticApi,
    scenario::tx_to_step::{bytes_annotated, u64_annotated},
    scenario_model::{BlockInfo, SetStateStep, U64Value},
};

use super::{SetStateBuilder, SetStateBuilderItem};

pub enum BlockItemTarget {
    Current,
    Previous,
    EpochStart,
}

pub struct BlockItem {
    target: BlockItemTarget,
    block_info: BlockInfo,
}

impl BlockItem {
    pub fn new_current() -> Self {
        BlockItem {
            target: BlockItemTarget::Current,
            block_info: BlockInfo::default(),
        }
    }

    pub fn new_prev() -> Self {
        BlockItem {
            target: BlockItemTarget::Previous,
            block_info: BlockInfo::default(),
        }
    }

    pub fn new_epoch_start() -> Self {
        BlockItem {
            target: BlockItemTarget::EpochStart,
            block_info: BlockInfo::default(),
        }
    }
}

impl SetStateBuilderItem for BlockItem {
    fn commit_to_step(&mut self, step: &mut SetStateStep) {
        let block_info = core::mem::take(&mut self.block_info);
        match self.target {
            BlockItemTarget::Current => {
                *step.current_block_info = Some(block_info);
            }
            BlockItemTarget::Previous => {
                *step.previous_block_info = Some(block_info);
            }
            BlockItemTarget::EpochStart => {
                *step.epoch_start_block_info = Some(block_info);
            }
        }
    }
}

impl SetStateBuilder<'_, BlockItem> {
    pub fn block_epoch<N>(mut self, block_epoch: N) -> Self
    where
        N: AnnotatedValue<ScenarioTxEnvData, u64>,
    {
        let env = self.new_env_data();
        let block_epoch_value = u64_annotated(&env, &block_epoch);

        self.item.block_info.block_epoch = Some(block_epoch_value);
        self
    }

    pub fn block_nonce<N>(mut self, block_nonce: N) -> Self
    where
        N: AnnotatedValue<ScenarioTxEnvData, u64>,
    {
        let env = self.new_env_data();
        let block_nonce_value = u64_annotated(&env, &block_nonce);

        self.item.block_info.block_nonce = Some(block_nonce_value);
        self
    }

    pub fn block_round<N>(mut self, block_round: N) -> Self
    where
        N: AnnotatedValue<ScenarioTxEnvData, u64>,
    {
        let env = self.new_env_data();
        let block_round_value = u64_annotated(&env, &block_round);

        self.item.block_info.block_round = Some(block_round_value);
        self
    }

    #[deprecated(since = "0.63.1", note = "Renamed to block_timestamp_seconds")]
    pub fn block_timestamp<N>(self, block_timestamp: N) -> Self
    where
        N: AnnotatedValue<ScenarioTxEnvData, TimestampSeconds>,
    {
        self.block_timestamp_seconds(block_timestamp)
    }

    /// Sets the current block timestamp, in seconds.
    pub fn block_timestamp_seconds<N>(mut self, block_timestamp: N) -> Self
    where
        N: AnnotatedValue<ScenarioTxEnvData, TimestampSeconds>,
    {
        let env = self.new_env_data();
        let annotation = block_timestamp.annotation(&env).to_string();
        self.item.block_info.block_timestamp = Some(U64Value {
            value: block_timestamp.to_value(&env).as_u64_seconds(),
            original: ValueSubTree::Str(annotation),
        });
        self
    }

    #[deprecated(since = "0.63.1", note = "Renamed to block_timestamp_millis")]
    pub fn block_timestamp_ms<N>(self, block_timestamp: N) -> Self
    where
        N: AnnotatedValue<ScenarioTxEnvData, TimestampMillis>,
    {
        self.block_timestamp_millis(block_timestamp)
    }

    /// Sets the current block timestamp, in milliseconds.
    pub fn block_timestamp_millis<N>(mut self, block_timestamp_ms: N) -> Self
    where
        N: AnnotatedValue<ScenarioTxEnvData, TimestampMillis>,
    {
        let env = self.new_env_data();
        let annotation = block_timestamp_ms.annotation(&env).to_string();
        self.item.block_info.block_timestamp_ms = Some(U64Value {
            value: block_timestamp_ms.to_value(&env).as_u64_millis(),
            original: ValueSubTree::Str(annotation),
        });
        self
    }

    pub fn block_random_seed<B>(mut self, block_random_seed: B) -> Self
    where
        B: AnnotatedValue<ScenarioTxEnvData, ManagedBuffer<StaticApi>>,
    {
        let env = self.new_env_data();
        let block_random_seed_value = bytes_annotated(&env, block_random_seed);

        self.item.block_info.block_random_seed = Some(block_random_seed_value);
        self
    }
}
