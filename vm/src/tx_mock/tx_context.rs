use crate::{
    tx_execution::BlockchainVMRef,
    types::VMAddress,
    world_mock::{AccountData, AccountEsdt, BlockchainState, FailingExecutor},
};
use num_bigint::BigUint;
use num_traits::Zero;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use super::{BlockchainRng, BlockchainUpdate, TxCache, TxInput, TxManagedTypes, TxResult};

pub struct TxContext {
    pub vm_ref: BlockchainVMRef,
    pub tx_input_box: Box<TxInput>,
    pub tx_cache: Arc<TxCache>,
    pub managed_types: Mutex<TxManagedTypes>,
    pub tx_result_cell: Mutex<TxResult>,
    pub b_rng: Mutex<BlockchainRng>,
}

impl TxContext {
    pub fn new(vm_ref: BlockchainVMRef, tx_input: TxInput, tx_cache: TxCache) -> Self {
        let b_rng = Mutex::new(BlockchainRng::new(&tx_input, &tx_cache));
        TxContext {
            vm_ref,
            tx_input_box: Box::new(tx_input),
            tx_cache: Arc::new(tx_cache),
            managed_types: Mutex::new(TxManagedTypes::new()),
            tx_result_cell: Mutex::new(TxResult::empty()),
            b_rng,
        }
    }

    pub fn dummy() -> Self {
        let tx_cache = TxCache::new(Arc::new(BlockchainState::default()));
        let contract_address = VMAddress::from([b'c'; 32]);
        tx_cache.insert_account(AccountData {
            address: contract_address.clone(),
            nonce: 0,
            egld_balance: BigUint::zero(),
            storage: HashMap::new(),
            esdt: AccountEsdt::default(),
            username: Vec::new(),
            contract_path: None,
            contract_owner: None,
            developer_rewards: BigUint::zero(),
        });

        let tx_input = TxInput {
            from: contract_address.clone(),
            to: contract_address,
            tx_hash: b"dummy...........................".into(),
            ..Default::default()
        };

        let b_rng = Mutex::new(BlockchainRng::new(&tx_input, &tx_cache));
        TxContext {
            vm_ref: BlockchainVMRef::new(Box::new(FailingExecutor)),
            tx_input_box: Box::new(tx_input),
            tx_cache: Arc::new(tx_cache),
            managed_types: Mutex::new(TxManagedTypes::new()),
            tx_result_cell: Mutex::new(TxResult::empty()),
            b_rng,
        }
    }

    pub fn input_ref(&self) -> &TxInput {
        self.tx_input_box.as_ref()
    }

    pub fn blockchain_cache(&self) -> &TxCache {
        &self.tx_cache
    }

    pub fn blockchain_cache_arc(&self) -> Arc<TxCache> {
        self.tx_cache.clone()
    }

    pub fn blockchain_ref(&self) -> &BlockchainState {
        self.tx_cache.blockchain_ref()
    }

    pub fn with_account<R, F>(&self, address: &VMAddress, f: F) -> R
    where
        F: FnOnce(&AccountData) -> R,
    {
        self.tx_cache.with_account(address, f)
    }

    pub fn with_contract_account<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&AccountData) -> R,
    {
        self.with_account(&self.tx_input_box.to, f)
    }

    pub fn with_account_mut<R, F>(&self, address: &VMAddress, f: F) -> R
    where
        F: FnOnce(&mut AccountData) -> R,
    {
        self.tx_cache.with_account_mut(address, f)
    }

    pub fn with_contract_account_mut<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut AccountData) -> R,
    {
        self.with_account_mut(&self.tx_input_box.to, f)
    }

    pub fn m_types_lock(&self) -> MutexGuard<TxManagedTypes> {
        self.managed_types.lock().unwrap()
    }

    pub fn result_lock(&self) -> MutexGuard<TxResult> {
        self.tx_result_cell.lock().unwrap()
    }

    pub fn extract_result(&self) -> TxResult {
        std::mem::replace(&mut *self.tx_result_cell.lock().unwrap(), TxResult::empty())
    }

    pub fn rng_lock(&self) -> MutexGuard<BlockchainRng> {
        self.b_rng.lock().unwrap()
    }

    pub fn create_new_contract(
        &self,
        new_address: &VMAddress,
        contract_path: Vec<u8>,
        contract_owner: VMAddress,
    ) {
        assert!(
            !self.tx_cache.blockchain_ref().account_exists(new_address),
            "Account already exists at deploy address."
        );

        self.tx_cache.insert_account(AccountData {
            address: new_address.clone(),
            nonce: 0,
            egld_balance: BigUint::zero(),
            storage: HashMap::new(),
            esdt: AccountEsdt::default(),
            username: Vec::new(),
            contract_path: Some(contract_path),
            contract_owner: Some(contract_owner),
            developer_rewards: BigUint::zero(),
        });
    }

    pub fn into_blockchain_updates(self) -> BlockchainUpdate {
        let tx_cache = Arc::try_unwrap(self.tx_cache).unwrap();
        tx_cache.into_blockchain_updates()
    }

    pub fn into_results(self) -> (TxResult, BlockchainUpdate) {
        let tx_cache = Arc::try_unwrap(self.tx_cache).unwrap();
        let tx_result = Mutex::into_inner(self.tx_result_cell).unwrap();
        let blockchain_updates = tx_cache.into_blockchain_updates();
        (tx_result, blockchain_updates)
    }
}

impl std::fmt::Debug for TxContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TxContext")
            .field("tx_input_box", &self.tx_input_box)
            .field("tx_cache", &self.tx_cache)
            .field("managed_types", &self.managed_types)
            .field("tx_result_cell", &self.tx_result_cell)
            .field("b_rng", &self.b_rng)
            .finish()
    }
}
