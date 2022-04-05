use crate::{
    num_bigint::BigUint,
    tx_mock::{BlockchainUpdate, TxCache},
    ContractMap,
};
use elrond_wasm::types::heap::Address;
use mandos::{
    interpret_trait::InterpreterContext, model::Scenario, value_interpreter::interpret_string,
};
use num_traits::Zero;
use std::{collections::HashMap, path::PathBuf, rc::Rc};

use super::{AccountData, BlockInfo};

const ELROND_REWARD_KEY: &[u8] = b"ELRONDreward";

#[derive(Debug)]
pub struct BlockchainMock {
    pub accounts: HashMap<Address, AccountData>,
    pub new_addresses: HashMap<(Address, u64), Address>,
    pub previous_block_info: BlockInfo,
    pub current_block_info: BlockInfo,
    pub contract_map: ContractMap,
    pub current_dir: PathBuf,
    pub mandos_trace: Scenario, // can be printed later - TODO: write the actual print
}

impl BlockchainMock {
    pub fn new() -> Self {
        BlockchainMock {
            accounts: HashMap::new(),
            new_addresses: HashMap::new(),
            previous_block_info: BlockInfo::new(),
            current_block_info: BlockInfo::new(),
            contract_map: ContractMap::default(),
            current_dir: std::env::current_dir().unwrap(),
            mandos_trace: Scenario::default(),
        }
    }
}

impl Default for BlockchainMock {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockchainMock {
    pub fn account_exists(&self, address: &Address) -> bool {
        self.accounts.contains_key(address)
    }

    pub fn contains_contract(&self, contract_path_expr: &str) -> bool {
        let contract_bytes = interpret_string(
            contract_path_expr,
            &InterpreterContext::new(self.current_dir.clone()),
        );

        self.contract_map.contains_contract(&contract_bytes)
    }

    pub fn commit_updates(self: &mut Rc<Self>, updates: BlockchainUpdate) {
        updates.apply(Rc::get_mut(self).unwrap());
    }

    pub fn commit_tx_cache(self: &mut Rc<Self>, tx_cache: TxCache) {
        self.commit_updates(tx_cache.into_blockchain_updates())
    }

    pub fn increase_account_nonce(self: &mut Rc<Self>, address: &Address) {
        let self_ref = Rc::get_mut(self).unwrap();
        let account = self_ref.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.nonce += 1;
    }

    pub fn subtract_tx_gas(self: &mut Rc<Self>, address: &Address, gas_limit: u64, gas_price: u64) {
        let self_ref = Rc::get_mut(self).unwrap();
        let account = self_ref.accounts.get_mut(address).unwrap_or_else(|| {
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

    pub fn increase_validator_reward(&mut self, address: &Address, amount: &BigUint) {
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

    pub(crate) fn with_borrowed_rc<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Rc<Self>) -> R,
    {
        let obj = std::mem::replace(self, Self::new());
        let mut state_rc = Rc::new(obj);
        let result = f(&mut state_rc);
        *self = Rc::try_unwrap(state_rc).unwrap();
        result
    }

    pub(crate) fn with_borrowed<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(Self) -> (R, Self),
    {
        let obj = std::mem::replace(self, Self::new());
        let (result, obj) = f(obj);
        *self = obj;
        result
    }
}
