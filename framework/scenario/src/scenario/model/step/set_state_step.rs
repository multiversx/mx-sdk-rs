use std::collections::BTreeMap;

use crate::scenario::model::{
    Account, AddressKey, AddressValue, BlockInfo, BytesValue, NewAddress, U64Value,
};

#[derive(Debug, Default, Clone)]
pub struct SetStateStep {
    pub comment: Option<String>,
    pub accounts: BTreeMap<AddressKey, Account>,
    pub new_addresses: Vec<NewAddress>,
    pub new_token_identifiers: Vec<String>,
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

    pub fn new_token_identifier<T>(mut self, token_identifier: T) -> Self
    where
        String: From<T>,
    {
        self.new_token_identifiers
            .push(String::from(token_identifier));
        self
    }

    pub fn block_epoch<N>(mut self, block_epoch_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_epoch = U64Value::from(block_epoch_expr);

        let mut block_info = self.current_block_info.unwrap_or_default();
        block_info.block_epoch = Some(block_epoch);
        self.current_block_info = Box::new(Some(block_info));
        self
    }

    pub fn block_nonce<N>(mut self, block_nonce_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_nonce = U64Value::from(block_nonce_expr);

        let mut block_info = self.current_block_info.unwrap_or_default();
        block_info.block_nonce = Some(block_nonce);
        self.current_block_info = Box::new(Some(block_info));
        self
    }

    pub fn block_round<N>(mut self, block_round_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_round = U64Value::from(block_round_expr);

        let mut block_info = self.current_block_info.unwrap_or_default();
        block_info.block_round = Some(block_round);
        self.current_block_info = Box::new(Some(block_info));
        self
    }

    pub fn block_timestamp<N>(mut self, block_timestamp_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_timestamp = U64Value::from(block_timestamp_expr);

        let mut block_info = self.current_block_info.unwrap_or_default();
        block_info.block_timestamp = Some(block_timestamp);
        self.current_block_info = Box::new(Some(block_info));
        self
    }

    pub fn block_random_seed<B>(mut self, block_random_seed_expr: B) -> Self
    where
        BytesValue: From<B>,
    {
        let block_random_seed = BytesValue::from(block_random_seed_expr);

        let mut block_info = self.current_block_info.unwrap_or_default();
        block_info.block_random_seed = Some(block_random_seed);
        self.current_block_info = Box::new(Some(block_info));
        self
    }

    pub fn prev_block_epoch<N>(mut self, block_epoch_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_epoch = U64Value::from(block_epoch_expr);

        let mut block_info = self.previous_block_info.unwrap_or_default();
        block_info.block_epoch = Some(block_epoch);
        self.previous_block_info = Box::new(Some(block_info));
        self
    }

    pub fn prev_block_nonce<N>(mut self, block_nonce_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_nonce = U64Value::from(block_nonce_expr);

        let mut block_info = self.previous_block_info.unwrap_or_default();
        block_info.block_nonce = Some(block_nonce);
        self.previous_block_info = Box::new(Some(block_info));
        self
    }

    pub fn prev_block_round<N>(mut self, block_round_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_round = U64Value::from(block_round_expr);

        let mut block_info = self.previous_block_info.unwrap_or_default();
        block_info.block_round = Some(block_round);
        self.previous_block_info = Box::new(Some(block_info));
        self
    }

    pub fn prev_block_timestamp<N>(mut self, block_timestamp_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_timestamp = U64Value::from(block_timestamp_expr);

        let mut block_info = self.previous_block_info.unwrap_or_default();
        block_info.block_timestamp = Some(block_timestamp);
        self.previous_block_info = Box::new(Some(block_info));
        self
    }

    pub fn prev_block_random_seed<B>(mut self, block_random_seed_expr: B) -> Self
    where
        BytesValue: From<B>,
    {
        let block_random_seed = BytesValue::from(block_random_seed_expr);

        let mut block_info = self.previous_block_info.unwrap_or_default();
        block_info.block_random_seed = Some(block_random_seed);
        self.previous_block_info = Box::new(Some(block_info));
        self
    }
}
