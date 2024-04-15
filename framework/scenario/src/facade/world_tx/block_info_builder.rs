use crate::{
    scenario::ScenarioRunner,
    scenario_model::{BlockInfo, BytesValue, SetStateStep, U64Value},
    ScenarioWorld,
};

pub struct BlockInfoBuilder<'w> {
    world: &'w mut ScenarioWorld,
    set_state_step: SetStateStep,
    current_block: BlockInfo,
    previous_block: BlockInfo,
}

impl<'w> BlockInfoBuilder<'w> {
    pub(crate) fn new(world: &'w mut ScenarioWorld) -> BlockInfoBuilder<'w> {
        let mut builder = BlockInfoBuilder {
            world,
            set_state_step: SetStateStep::new(),
            current_block: BlockInfo::default(),
            previous_block: BlockInfo::default(),
        };
        builder
    }

    // Forces value drop and commit block info.
    pub fn commit(self) {}

    /// Finished and sets all account in the blockchain mock.
    fn commit_block_info(&mut self) {
        self.add_current_block_info();
        self.world.run_set_state_step(&self.set_state_step);
    }

    fn add_current_block_info(&mut self) {
        self.set_state_step.previous_block_info =
            Box::new(Some(core::mem::take(&mut self.previous_block)));
        self.set_state_step.current_block_info =
            Box::new(Some(core::mem::take(&mut self.current_block)));
    }

    pub fn block_epoch<N>(mut self, block_epoch_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_epoch = U64Value::from(block_epoch_expr);

        self.current_block.block_epoch = Some(block_epoch);
        self
    }

    pub fn block_nonce<N>(mut self, block_nonce_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_nonce = U64Value::from(block_nonce_expr);

        self.current_block.block_nonce = Some(block_nonce);
        self
    }

    pub fn block_round<N>(mut self, block_round_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_round = U64Value::from(block_round_expr);

        self.current_block.block_round = Some(block_round);
        self
    }

    pub fn block_timestamp<N>(mut self, block_timestamp_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_timestamp = U64Value::from(block_timestamp_expr);

        self.current_block.block_timestamp = Some(block_timestamp);
        self
    }

    pub fn block_random_seed<B>(mut self, block_random_seed_expr: B) -> Self
    where
        BytesValue: From<B>,
    {
        let block_random_seed = BytesValue::from(block_random_seed_expr);

        self.current_block.block_random_seed = Some(block_random_seed);
        self
    }

    pub fn prev_block_epoch<N>(mut self, block_epoch_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let prev_block_epoch = U64Value::from(block_epoch_expr);

        self.previous_block.block_epoch = Some(prev_block_epoch);
        self
    }

    pub fn prev_block_nonce<N>(mut self, block_nonce_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let prev_block_nonce = U64Value::from(block_nonce_expr);

        self.previous_block.block_nonce = Some(prev_block_nonce);
        self
    }

    pub fn prev_block_round<N>(mut self, block_round_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let prev_block_round = U64Value::from(block_round_expr);

        self.previous_block.block_round = Some(prev_block_round);
        self
    }

    pub fn prev_block_timestamp<N>(mut self, block_timestamp_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let prev_block_timestamp = U64Value::from(block_timestamp_expr);

        self.previous_block.block_timestamp = Some(prev_block_timestamp);
        self
    }

    pub fn prev_block_random_seed<B>(mut self, block_random_seed_expr: B) -> Self
    where
        BytesValue: From<B>,
    {
        let prev_block_random_seed = BytesValue::from(block_random_seed_expr);

        self.previous_block.block_random_seed = Some(prev_block_random_seed);
        self
    }
}

impl Drop for BlockInfoBuilder<'_> {
    fn drop(&mut self) {
        self.commit_block_info();
    }
}
