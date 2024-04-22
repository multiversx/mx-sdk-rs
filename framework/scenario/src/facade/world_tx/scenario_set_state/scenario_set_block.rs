use crate::scenario_model::{BlockInfo, BytesValue, SetStateStep, U64Value};

use super::{SetStateBuilder, SetStateBuilderItem};

pub enum BlockItemTarget {
    Current,
    Previous,
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
}

impl SetStateBuilderItem for BlockItem {
    fn commit_to_step(&mut self, step: &mut SetStateStep) {
        let block_info = core::mem::take(&mut self.block_info);
        match self.target {
            BlockItemTarget::Current => {
                step.current_block_info = Box::new(Some(block_info));
            },
            BlockItemTarget::Previous => {
                step.previous_block_info = Box::new(Some(block_info));
            },
        }
    }
}

impl<'w> SetStateBuilder<'w, BlockItem> {
    pub fn block_epoch<N>(mut self, block_epoch_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_epoch = U64Value::from(block_epoch_expr);

        self.item.block_info.block_epoch = Some(block_epoch);
        self
    }

    pub fn block_nonce<N>(mut self, block_nonce_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_nonce = U64Value::from(block_nonce_expr);

        self.item.block_info.block_nonce = Some(block_nonce);
        self
    }

    pub fn block_round<N>(mut self, block_round_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_round = U64Value::from(block_round_expr);

        self.item.block_info.block_round = Some(block_round);
        self
    }

    pub fn block_timestamp<N>(mut self, block_timestamp_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_timestamp = U64Value::from(block_timestamp_expr);

        self.item.block_info.block_timestamp = Some(block_timestamp);
        self
    }

    pub fn block_random_seed<B>(mut self, block_random_seed_expr: B) -> Self
    where
        BytesValue: From<B>,
    {
        let block_random_seed = BytesValue::from(block_random_seed_expr);

        self.item.block_info.block_random_seed = Some(block_random_seed);
        self
    }
}
