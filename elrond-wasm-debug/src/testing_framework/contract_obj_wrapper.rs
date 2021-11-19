use std::{collections::HashMap, marker::PhantomData, rc::Rc};

use elrond_wasm::{
    contract_base::ContractBase,
    types::{Address, H256},
};

use crate::{
    rust_biguint,
    tx_mock::{TxCache, TxInput},
    world_mock::{AccountData, AccountEsdt},
    BlockchainMock, DebugApi,
};

use super::AddressFactory;

pub struct ContractObjWrapper<
    CB: ContractBase<Api = DebugApi>,
    ContractObjBuilder: Fn(DebugApi) -> CB,
> {
    address_factory: AddressFactory,
    obj_builder: ContractObjBuilder,
    b_mock: BlockchainMock,
    _phantom: PhantomData<CB>,
}

pub enum StateChange {
    Commit,
    Revert,
}

impl<CB, ContractObjBuilder> ContractObjWrapper<CB, ContractObjBuilder>
where
    CB: ContractBase<Api = DebugApi>,
    ContractObjBuilder: Fn(DebugApi) -> CB,
{
    pub fn new(obj_builder: ContractObjBuilder) -> Self {
        ContractObjWrapper {
            address_factory: AddressFactory::new(),
            obj_builder,
            b_mock: BlockchainMock::new(),
            _phantom: PhantomData,
        }
    }

    pub fn check_egld_balance(&self, address: &Address, expected_balance: &num_bigint::BigUint) {
        let actual_balance = match &self.b_mock.accounts.get(address) {
            Some(acc) => acc.egld_balance.clone(),
            None => rust_biguint!(0),
        };

        assert_eq!(&actual_balance, expected_balance);
    }
}

impl<CB, ContractObjBuilder> ContractObjWrapper<CB, ContractObjBuilder>
where
    CB: ContractBase<Api = DebugApi>,
    ContractObjBuilder: Fn(DebugApi) -> CB,
{
    pub fn create_user_account(&mut self, egld_balance: &num_bigint::BigUint) -> Address {
        let address = self.address_factory.new_address();
        self.b_mock.add_account(AccountData {
            address: address.clone(),
            nonce: 0,
            egld_balance: egld_balance.clone(),
            esdt: AccountEsdt::default(),
            storage: HashMap::new(),
            username: Vec::new(),
            contract_path: None,
            contract_owner: None,
        });

        address
    }

    pub fn create_sc_account(
        &mut self,
        egld_balance: &num_bigint::BigUint,
        owner: Option<&Address>,
    ) -> Address {
        let address = self.address_factory.new_sc_address();
        self.b_mock.add_account(AccountData {
            address: address.clone(),
            nonce: 0,
            egld_balance: egld_balance.clone(),
            esdt: AccountEsdt::default(),
            storage: HashMap::new(),
            username: Vec::new(),
            contract_path: None,
            contract_owner: owner.map(|owner_ref| owner_ref.clone()),
        });

        address
    }

    pub fn set_block_epoch(&mut self, block_epoch: u64) {
        self.b_mock.current_block_info.block_epoch = block_epoch;
    }

    pub fn set_block_nonce(&mut self, block_nonce: u64) {
        self.b_mock.current_block_info.block_nonce = block_nonce;
    }

    pub fn set_block_random_seed(&mut self, block_random_seed: Box<[u8; 48]>) {
        self.b_mock.current_block_info.block_random_seed = block_random_seed;
    }

    pub fn set_block_round(&mut self, block_round: u64) {
        self.b_mock.current_block_info.block_round = block_round;
    }

    pub fn set_block_timestamp(&mut self, block_timestamp: u64) {
        self.b_mock.current_block_info.block_timestamp = block_timestamp;
    }

    pub fn set_prev_block_epoch(&mut self, block_epoch: u64) {
        self.b_mock.previous_block_info.block_epoch = block_epoch;
    }

    pub fn set_prev_block_nonce(&mut self, block_nonce: u64) {
        self.b_mock.previous_block_info.block_nonce = block_nonce;
    }

    pub fn set_prev_block_random_seed(&mut self, block_random_seed: Box<[u8; 48]>) {
        self.b_mock.previous_block_info.block_random_seed = block_random_seed;
    }

    pub fn set_prev_block_round(&mut self, block_round: u64) {
        self.b_mock.previous_block_info.block_round = block_round;
    }

    pub fn set_prev_block_timestamp(&mut self, block_timestamp: u64) {
        self.b_mock.previous_block_info.block_timestamp = block_timestamp;
    }
}

impl<CB, ContractObjBuilder> ContractObjWrapper<CB, ContractObjBuilder>
where
    CB: ContractBase<Api = DebugApi>,
    ContractObjBuilder: Fn(DebugApi) -> CB,
{
    pub fn execute_tx<TxFn: FnOnce(&CB) -> StateChange>(
        self,
        caller: &Address,
        sc_address: &Address,
        egld_payment: &num_bigint::BigUint,
        tx_fn: TxFn,
    ) -> Self {
        let rc_b_mock = Rc::new(self.b_mock);
        let tx_cache = TxCache::new(rc_b_mock.clone());

        if egld_payment > &rust_biguint!(0) {
            tx_cache.subtract_egld_balance(caller, egld_payment);
            tx_cache.increase_egld_balance(sc_address, egld_payment);
        }

        let tx_input = build_tx_input(caller, sc_address, egld_payment);
        let debug_api = DebugApi::new(tx_input, tx_cache);
        let sc = (self.obj_builder)(debug_api);

        let state_change = tx_fn(&sc);

        let api_after_exec = into_api(sc);
        let updates = api_after_exec.into_blockchain_updates();
        let mut new_b_mock = Rc::try_unwrap(rc_b_mock).unwrap();

        match state_change {
            StateChange::Commit => {
                updates.apply(&mut new_b_mock);
            },
            StateChange::Revert => {},
        }

        Self {
            address_factory: self.address_factory,
            obj_builder: self.obj_builder,
            b_mock: new_b_mock,
            _phantom: PhantomData,
        }
    }

    pub fn execute_query<TxFn: FnOnce(&CB)>(self, sc_address: &Address, query_fn: TxFn) -> Self {
        self.execute_tx(sc_address, sc_address, &rust_biguint!(0), |sc| {
            query_fn(sc);
            StateChange::Revert
        })
    }
}

fn into_api<CB: ContractBase<Api = DebugApi>>(sc_obj: CB) -> DebugApi {
    sc_obj.raw_vm_api()
}

fn build_tx_input(caller: &Address, dest: &Address, egld_value: &num_bigint::BigUint) -> TxInput {
    TxInput {
        from: caller.clone(),
        to: dest.clone(),
        egld_value: egld_value.clone(),
        esdt_values: Vec::new(),
        func_name: Vec::new(),
        args: Vec::new(),
        gas_limit: u64::MAX,
        gas_price: 0,
        tx_hash: H256::zero(),
    }
}
