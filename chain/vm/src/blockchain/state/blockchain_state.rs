use multiversx_chain_core::types::CodeMetadata;
use num_bigint::BigUint;
use num_traits::Zero;
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    blockchain::reserved::STORAGE_REWARD_KEY, host::context::BlockchainUpdate,
    system_sc::ESDT_SYSTEM_SC_ADDRESS, types::VMAddress,
};

use super::{AccountData, BlockConfig};

#[derive(Clone)]
pub struct BlockchainState {
    pub accounts: HashMap<VMAddress, AccountData>,
    pub new_addresses: HashMap<(VMAddress, u64), VMAddress>,
    pub block_config: BlockConfig,
    pub new_token_identifiers: Vec<String>,
}

impl Default for BlockchainState {
    fn default() -> Self {
        let mut state = Self {
            accounts: Default::default(),
            new_addresses: Default::default(),
            block_config: Default::default(),
            new_token_identifiers: Default::default(),
        };

        // pre-populating system SC(s)
        state.add_empty_account(ESDT_SYSTEM_SC_ADDRESS);

        state
    }
}

impl BlockchainState {
    pub fn commit_updates(&mut self, updates: BlockchainUpdate) {
        updates.apply(self);
    }

    pub fn account_exists(&self, address: &VMAddress) -> bool {
        self.accounts.contains_key(address)
    }

    pub fn increase_account_nonce(&mut self, address: &VMAddress) {
        let account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Account not found: {address}"));
        account.nonce += 1;
    }

    pub fn subtract_tx_gas(&mut self, address: &VMAddress, gas_limit: u64, gas_price: u64) {
        let account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Account not found: {address}"));
        let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
        assert!(
            account.egld_balance >= gas_cost,
            "Not enough balance to pay gas upfront"
        );
        account.egld_balance -= &gas_cost;
    }

    pub fn increase_validator_reward(&mut self, address: &VMAddress, amount: &BigUint) {
        let account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Account not found: {address}"));
        account.egld_balance += amount;
        let mut storage_v_rew =
            if let Some(old_storage_value) = account.storage.get(STORAGE_REWARD_KEY) {
                BigUint::from_bytes_be(old_storage_value)
            } else {
                BigUint::zero()
            };
        storage_v_rew += amount;
        account
            .storage
            .insert(STORAGE_REWARD_KEY.to_vec(), storage_v_rew.to_bytes_be());
    }

    pub fn put_new_token_identifier(&mut self, token_identifier: String) {
        self.new_token_identifiers.push(token_identifier)
    }

    pub fn get_new_token_identifiers(&self) -> Vec<String> {
        self.new_token_identifiers.clone()
    }

    pub fn update_new_token_identifiers(&mut self, token_identifiers: Vec<String>) {
        self.new_token_identifiers = token_identifiers;
    }

    fn add_empty_account(&mut self, address: VMAddress) {
        self.accounts.insert(
            address.clone(),
            AccountData {
                address,
                nonce: 0,
                egld_balance: Default::default(),
                esdt: Default::default(),
                username: Vec::new(),
                storage: HashMap::new(),
                contract_path: None,
                code_metadata: CodeMetadata::empty(),
                contract_owner: None,
                developer_rewards: BigUint::zero(),
            },
        );
    }
}

impl Debug for BlockchainState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlockchainState")
            .field("accounts", &self.accounts)
            .field("new_addresses", &self.new_addresses)
            .field("block_config", &self.block_config)
            .finish()
    }
}

#[derive(Default, Clone)]
pub struct BlockchainStateRef(Arc<BlockchainState>);

impl BlockchainStateRef {
    pub fn mut_state(&mut self) -> &mut BlockchainState {
        Arc::get_mut(&mut self.0).expect("cannot change state, since object is currently shared")
    }

    pub fn get_arc(&self) -> Arc<BlockchainState> {
        self.0.clone()
    }
}

impl Deref for BlockchainStateRef {
    type Target = BlockchainState;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl DerefMut for BlockchainStateRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::get_mut(&mut self.0).expect("cannot change state, since object is currently shared")
    }
}
