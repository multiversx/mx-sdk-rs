use crate::{
    tx_execution::{init_builtin_functions, BuiltinFunctionMap},
    tx_mock::BlockchainUpdate,
    types::VMAddress,
};
use multiversx_chain_vm_executor::Executor;
use num_bigint::BigUint;
use num_traits::Zero;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    rc::Rc,
};

use super::{AccountData, BlockInfo, FailingExecutor};

const ELROND_REWARD_KEY: &[u8] = b"ELRONDreward";

pub struct BlockchainMock {
    pub accounts: HashMap<VMAddress, AccountData>,
    pub builtin_functions: Rc<BuiltinFunctionMap>,
    pub new_addresses: HashMap<(VMAddress, u64), VMAddress>,
    pub new_token_identifiers: VecDeque<String>,
    pub previous_block_info: BlockInfo,
    pub current_block_info: BlockInfo,
    pub executor: Box<dyn Executor>,
}

impl BlockchainMock {
    pub fn new(executor: Box<dyn Executor>) -> Self {
        BlockchainMock {
            accounts: HashMap::new(),
            builtin_functions: Rc::new(init_builtin_functions()),
            new_addresses: HashMap::new(),
            new_token_identifiers: VecDeque::new(),
            previous_block_info: BlockInfo::new(),
            current_block_info: BlockInfo::new(),
            executor,
        }
    }

    pub fn put_new_token_identifier(&mut self, token_identifier: String) {
        self.new_token_identifiers.push_back(token_identifier)
    }

    pub fn get_new_token_identifiers(&self) -> VecDeque<String> {
        self.new_token_identifiers.clone()
    }

    pub fn update_new_token_identifiers(&mut self, token_identifiers: VecDeque<String>) {
        self.new_token_identifiers = token_identifiers;
    }

    pub fn account_exists(&self, address: &VMAddress) -> bool {
        self.accounts.contains_key(address)
    }

    pub fn commit_updates(&mut self, updates: BlockchainUpdate) {
        updates.apply(self);
    }

    pub fn increase_account_nonce(&mut self, address: &VMAddress) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.nonce += 1;
    }

    pub fn subtract_tx_gas(&mut self, address: &VMAddress, gas_limit: u64, gas_price: u64) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
        assert!(
            account.egld_balance >= gas_cost,
            "Not enough balance to pay gas upfront"
        );
        account.egld_balance -= &gas_cost;
    }

    pub fn increase_validator_reward(&mut self, address: &VMAddress, amount: &BigUint) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.egld_balance += amount;
        let mut storage_v_rew =
            if let Some(old_storage_value) = account.storage.get(ELROND_REWARD_KEY) {
                BigUint::from_bytes_be(old_storage_value)
            } else {
                BigUint::zero()
            };
        storage_v_rew += amount;
        account
            .storage
            .insert(ELROND_REWARD_KEY.to_vec(), storage_v_rew.to_bytes_be());
    }

    pub fn with_borrowed<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(Self) -> (R, Self),
    {
        let obj = std::mem::take(self);
        let (result, obj) = f(obj);
        *self = obj;
        result
    }
}

impl Default for BlockchainMock {
    fn default() -> Self {
        Self::new(Box::new(FailingExecutor))
    }
}

impl Debug for BlockchainMock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlockchainMock")
            .field("accounts", &self.accounts)
            .field("new_addresses", &self.new_addresses)
            .field("current_block_info", &self.current_block_info)
            .finish()
    }
}
