use std::collections::BTreeMap;

use crate::scenario::model::{
    Account, AddressKey, AddressValue, BlockInfo, BytesValue, NewAddress, U64Value,
};

#[derive(Debug, Default)]
pub struct SetStateStep {
    pub comment: Option<String>,
    pub accounts: BTreeMap<AddressKey, Account>,
    pub new_addresses: Vec<NewAddress>,
    pub block_hashes: Vec<BytesValue>,
    pub previous_block_info: Box<Option<BlockInfo>>,
    pub current_block_info: Box<Option<BlockInfo>>,
}

impl SetStateStep {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put_account<A>(mut self, address_expr: A, account: Account) -> Self
    where
        AddressKey: From<A>,
    {
        let address_key = AddressKey::from(address_expr);
        self.accounts.insert(address_key, account);
        self
    }

    pub fn new_address<CA, NA>(
        mut self,
        creator_address_expr: CA,
        creator_nonce_expr: u64,
        new_address_expr: NA,
    ) -> Self
    where
        AddressValue: From<CA>,
        AddressValue: From<NA>,
    {
        self.new_addresses.push(NewAddress {
            creator_address: AddressValue::from(creator_address_expr),
            creator_nonce: U64Value::from(creator_nonce_expr),
            new_address: AddressValue::from(new_address_expr),
        });
        self
    }

    pub fn block_epoch<N>(mut self, block_epoch_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_epoch = U64Value::from(block_epoch_expr);

        if let Some(block_info) = &mut *self.current_block_info {
            block_info.block_epoch = Some(block_epoch);
        } else {
            *self.current_block_info = Some(BlockInfo {
                block_epoch: Some(block_epoch),
                ..Default::default()
            });
        }

        self
    }

    pub fn block_nonce<N>(mut self, block_nonce_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_nonce = U64Value::from(block_nonce_expr);

        if let Some(block_info) = &mut *self.current_block_info {
            block_info.block_nonce = Some(block_nonce);
        } else {
            *self.current_block_info = Some(BlockInfo {
                block_nonce: Some(block_nonce),
                ..Default::default()
            });
        }

        self
    }

    pub fn block_round<N>(mut self, block_round_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_round = U64Value::from(block_round_expr);

        if let Some(block_info) = &mut *self.current_block_info {
            block_info.block_round = Some(block_round);
        } else {
            *self.current_block_info = Some(BlockInfo {
                block_round: Some(block_round),
                ..Default::default()
            });
        }

        self
    }

    pub fn block_timestamp<N>(mut self, block_timestamp_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_timestamp = U64Value::from(block_timestamp_expr);

        if let Some(block_info) = &mut *self.current_block_info {
            block_info.block_timestamp = Some(block_timestamp);
        } else {
            *self.current_block_info = Some(BlockInfo {
                block_timestamp: Some(block_timestamp),
                ..Default::default()
            });
        }

        self
    }

    pub fn block_random_seed<B>(mut self, block_random_seed_expr: B) -> Self
    where
        BytesValue: From<B>,
    {
        let block_random_seed = BytesValue::from(block_random_seed_expr);

        if let Some(block_info) = &mut *self.current_block_info {
            block_info.block_random_seed = Some(block_random_seed);
        } else {
            *self.current_block_info = Some(BlockInfo {
                block_random_seed: Some(block_random_seed),
                ..Default::default()
            });
        }

        self
    }
}
