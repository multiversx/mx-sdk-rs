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

    pub fn check_egkd_balance(&self, address: &Address, expected_balance: &num_bigint::BigUint) {
        let actual_balance = match &self.b_mock.accounts.get(address) {
            Some(acc) => acc.egld_balance.clone(),
            None => rust_biguint!(0),
        };

        assert_eq!(&actual_balance, expected_balance);
    }

    pub fn check_state<F: Fn(DebugApi)>(&self, _func: F) {
        /*
        match &self.api {
            Some(api) => func(api.clone()),
            None => panic!("Api instance not created yet"),
        }
        */
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

    pub fn execute_tx<TxFn: Fn(&CB)>(
        self,
        caller: &Address,
        sc_address: &Address,
        egld_payment: &num_bigint::BigUint,
        tx_fn: TxFn,
    ) -> Self {
        let rc_b_mock = Rc::new(self.b_mock);
        let tx_cache = TxCache::new(rc_b_mock.clone());

        if egld_payment > &num_bigint::BigUint::from(0u32) {
            tx_cache.subtract_egld_balance(caller, egld_payment);
            tx_cache.increase_egld_balance(sc_address, egld_payment);
        }

        let tx_input = TxInput {
            from: caller.clone(),
            to: sc_address.clone(),
            egld_value: egld_payment.clone(),
            esdt_values: Vec::new(),
            func_name: Vec::new(),
            args: Vec::new(),
            gas_limit: u64::MAX,
            gas_price: 0,
            tx_hash: H256::zero(),
        };
        let debug_api = DebugApi::new(tx_input, tx_cache);
        let sc = (self.obj_builder)(debug_api);

        tx_fn(&sc);

        let api_after_exec = into_api(sc);
        let updates = api_after_exec.into_blockchain_updates();
        let mut new_b_mock = Rc::try_unwrap(rc_b_mock).unwrap();
        updates.apply(&mut new_b_mock);

        Self {
            address_factory: self.address_factory,
            obj_builder: self.obj_builder,
            b_mock: new_b_mock,
            _phantom: PhantomData,
        }
    }

    pub fn execute_query<TxFn: Fn(&CB)>(self, sc_address: &Address, query_fn: TxFn) -> Self {
        let rc_b_mock = Rc::new(self.b_mock);
        let tx_cache = TxCache::new(rc_b_mock.clone());

        let tx_input = TxInput {
            from: sc_address.clone(),
            to: sc_address.clone(),
            egld_value: rust_biguint!(0),
            esdt_values: Vec::new(),
            func_name: Vec::new(),
            args: Vec::new(),
            gas_limit: u64::MAX,
            gas_price: 0,
            tx_hash: H256::zero(),
        };
        let debug_api = DebugApi::new(tx_input, tx_cache);
        let sc = (self.obj_builder)(debug_api);

        query_fn(&sc);

        // we don't actually apply the updates, we only make sure to destroy the API object
        let api_after_exec = into_api(sc);
        let _ = api_after_exec.into_blockchain_updates();
        let new_b_mock = Rc::try_unwrap(rc_b_mock).unwrap();

        Self {
            address_factory: self.address_factory,
            obj_builder: self.obj_builder,
            b_mock: new_b_mock,
            _phantom: PhantomData,
        }
    }

    // pub fn set_storage(&mut self, address: Address, key: ?, value: dyn TopEncode)
}

fn into_api<CB: ContractBase<Api = DebugApi>>(sc_obj: CB) -> DebugApi {
    sc_obj.raw_vm_api()
}
