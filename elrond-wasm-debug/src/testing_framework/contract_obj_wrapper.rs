use std::{collections::HashMap, marker::PhantomData, path::PathBuf, rc::Rc, str::FromStr};

use elrond_wasm::{
    contract_base::{CallableContract, ContractBase},
    types::{Address, EsdtLocalRole, H256},
};

use crate::{
    rust_biguint,
    testing_framework::bytes_to_hex,
    tx_mock::{TxCache, TxContext, TxContextStack, TxInput, TxInputESDT},
    world_mock::{AccountData, AccountEsdt, EsdtInstanceMetadata},
    BlockchainMock, DebugApi,
};

use super::{
    tx_mandos::{ScCallMandos, TxExpectMandos},
    AddressFactory, MandosGenerator, ScQueryMandos,
};

pub struct ContractObjWrapper<
    CB: ContractBase<Api = DebugApi> + CallableContract<DebugApi> + 'static,
    ContractObjBuilder: 'static + Copy + Fn(DebugApi) -> CB,
> {
    address_factory: AddressFactory,
    obj_builders: HashMap<Address, ContractObjBuilder>,
    b_mock: BlockchainMock,
    address_to_code_path: HashMap<Address, Vec<u8>>,
    mandos_generator: MandosGenerator,
    workspace_path: PathBuf,
    _phantom: PhantomData<CB>,
}

pub enum StateChange {
    Commit,
    Revert,
}

impl<CB, ContractObjBuilder> ContractObjWrapper<CB, ContractObjBuilder>
where
    CB: ContractBase<Api = DebugApi> + CallableContract<DebugApi> + 'static,
    ContractObjBuilder: 'static + Copy + Fn(DebugApi) -> CB,
{
    pub fn new() -> Self {
        let mut current_dir = std::env::current_dir().unwrap();
        current_dir.push(PathBuf::from_str("mandos/").unwrap());

        ContractObjWrapper {
            address_factory: AddressFactory::new(),
            obj_builders: HashMap::new(),
            b_mock: BlockchainMock::new(),
            address_to_code_path: HashMap::new(),
            mandos_generator: MandosGenerator::new(),
            workspace_path: current_dir,
            _phantom: PhantomData,
        }
    }

    pub fn write_mandos_output(self, file_path: &str) {
        self.mandos_generator.write_mandos_output(file_path);
    }

    pub fn check_egld_balance(&self, address: &Address, expected_balance: &num_bigint::BigUint) {
        let actual_balance = match &self.b_mock.accounts.get(address) {
            Some(acc) => acc.egld_balance.clone(),
            None => rust_biguint!(0),
        };

        assert_eq!(
            expected_balance,
            &actual_balance,
            "EGLD balance mismatch for address {}. Expected: {}, have {}",
            address_to_hex(address),
            expected_balance,
            actual_balance
        );
    }

    pub fn check_esdt_balance(
        &self,
        address: &Address,
        token_id: &[u8],
        expected_balance: &num_bigint::BigUint,
    ) {
        let actual_balance = match &self.b_mock.accounts.get(address) {
            Some(acc) => acc.esdt.get_esdt_balance(token_id, 0),
            None => rust_biguint!(0),
        };

        assert_eq!(
            expected_balance,
            &actual_balance,
            "ESDT balance mismatch for address {}. Expected: {}, have {}",
            address_to_hex(address),
            expected_balance,
            actual_balance
        );
    }

    pub fn check_nft_balance<T: elrond_wasm::elrond_codec::TopEncode>(
        &self,
        address: &Address,
        token_id: &[u8],
        nonce: u64,
        expected_balance: &num_bigint::BigUint,
        expected_attributes: &T,
    ) {
        let actual_attributes = match &self.b_mock.accounts.get(address) {
            Some(acc) => {
                let esdt_data = acc.esdt.get_by_identifier_or_default(token_id);
                let opt_instance = esdt_data.instances.get_by_nonce(nonce);

                match opt_instance {
                    Some(instance) => {
                        assert_eq!(
                            expected_balance,
                            &instance.balance,
                            "ESDT NFT balance mismatch for address {}. Expected: {}, have {}",
                            address_to_hex(address),
                            expected_balance,
                            instance.balance
                        );

                        instance.metadata.attributes.clone()
                    },
                    None => Vec::new(),
                }
            },
            None => Vec::new(),
        };

        let serialized_expected = serialize_attributes(expected_attributes);
        assert_eq!(
            &serialized_expected,
            &actual_attributes,
            "ESDT NFT attributes mismatch for address {}. Expected: {}, have {}",
            address_to_hex(address),
            bytes_to_hex(&serialized_expected),
            bytes_to_hex(&actual_attributes),
        );
    }

    /*
    pub fn check_nft_balance_with_properties(
        &self,
        address: &Address,
        token_id: &[u8],
        nonce: u64,
        expected_balance: &num_bigint::BigUint,
    ) {
    }
    */
}

impl<CB, ContractObjBuilder> ContractObjWrapper<CB, ContractObjBuilder>
where
    CB: ContractBase<Api = DebugApi> + CallableContract<DebugApi> + 'static,
    ContractObjBuilder: 'static + Copy + Fn(DebugApi) -> CB,
{
    pub fn create_user_account(&mut self, egld_balance: &num_bigint::BigUint) -> Address {
        let address = self.address_factory.new_address();
        self.create_account_raw(&address, egld_balance, None, None, None);

        address
    }

    pub fn create_sc_account(
        &mut self,
        egld_balance: &num_bigint::BigUint,
        owner: Option<&Address>,
        obj_builder: ContractObjBuilder,
        contract_wasm_path: &str,
    ) -> Address {
        let address = self.address_factory.new_sc_address();

        let mut wasm_full_path = std::env::current_dir().unwrap();
        wasm_full_path.push(PathBuf::from_str(contract_wasm_path).unwrap());

        let path_diff =
            pathdiff::diff_paths(wasm_full_path.clone(), self.workspace_path.clone()).unwrap();
        let path_str = path_diff.to_str().unwrap();
        let path_bytes = path_str.as_bytes().to_vec();

        self.address_to_code_path
            .insert(address.clone(), path_bytes);

        let wasm_full_path_as_expr = "file:".to_owned() + &wasm_full_path.to_str().unwrap();
        let contract_bytes = mandos::value_interpreter::interpret_string(
            &wasm_full_path_as_expr,
            &mandos::interpret_trait::InterpreterContext::new(std::path::PathBuf::new()),
        );

        let wasm_relative_path_expr = "file:".to_owned() + path_str;
        self.create_account_raw(
            &address,
            egld_balance,
            owner,
            Some(contract_bytes), // Some(contract_path_expr.as_bytes().to_vec())
            Some(wasm_relative_path_expr.as_bytes().to_vec()),
        );
        let _ = self.obj_builders.insert(address.clone(), obj_builder);

        if !self.b_mock.contains_contract(&wasm_full_path_as_expr) {
            let closure = convert_full_fn(obj_builder);
            self.b_mock
                .register_contract(&wasm_full_path_as_expr, closure);
        }

        address
    }

    pub fn create_account_raw(
        &mut self,
        address: &Address,
        egld_balance: &num_bigint::BigUint,
        owner: Option<&Address>,
        sc_identifier: Option<Vec<u8>>,
        sc_mandos_path_expr: Option<Vec<u8>>,
    ) {
        let acc_data = AccountData {
            address: address.clone(),
            nonce: 0,
            egld_balance: egld_balance.clone(),
            esdt: AccountEsdt::default(),
            storage: HashMap::new(),
            username: Vec::new(),
            contract_path: sc_identifier,
            contract_owner: owner.cloned(),
        };
        self.mandos_generator
            .set_account(address, &acc_data, sc_mandos_path_expr);

        self.b_mock.add_account(acc_data);
    }

    pub fn set_egld_balance(&mut self, address: &Address, balance: &num_bigint::BigUint) {
        match self.b_mock.accounts.get_mut(address) {
            Some(acc) => {
                acc.egld_balance = balance.clone();

                let opt_contract_path = self.address_to_code_path.get(address);
                self.mandos_generator
                    .set_account(address, acc, opt_contract_path.cloned());
            },

            None => panic!(
                "set_egld_balance: Account {:?} does not exist",
                address_to_hex(address)
            ),
        }
    }

    pub fn set_esdt_balance(
        &mut self,
        address: &Address,
        token_id: &[u8],
        balance: &num_bigint::BigUint,
    ) {
        match self.b_mock.accounts.get_mut(address) {
            Some(acc) => {
                acc.esdt.set_esdt_balance(
                    token_id.to_vec(),
                    0,
                    balance,
                    EsdtInstanceMetadata::default(),
                );

                let opt_contract_path = self.address_to_code_path.get(address);
                self.mandos_generator
                    .set_account(address, acc, opt_contract_path.cloned());
            },
            None => panic!(
                "set_esdt_balance: Account {:?} does not exist",
                address_to_hex(address)
            ),
        }
    }

    pub fn set_nft_balance<T: elrond_wasm::elrond_codec::TopEncode>(
        &mut self,
        address: &Address,
        token_id: &[u8],
        nonce: u64,
        balance: &num_bigint::BigUint,
        attributes: &T,
    ) {
        self.set_nft_balance_all_properties(
            address, token_id, nonce, balance, attributes, 0, None, None, None, None,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_nft_balance_all_properties<T: elrond_wasm::elrond_codec::TopEncode>(
        &mut self,
        address: &Address,
        token_id: &[u8],
        nonce: u64,
        balance: &num_bigint::BigUint,
        attributes: &T,
        royalties: u64,
        creator: Option<&Address>,
        name: Option<&[u8]>,
        hash: Option<&[u8]>,
        uri: Option<&[u8]>,
    ) {
        match self.b_mock.accounts.get_mut(address) {
            Some(acc) => {
                acc.esdt.set_esdt_balance(
                    token_id.to_vec(),
                    nonce,
                    balance,
                    EsdtInstanceMetadata {
                        creator: creator.cloned(),
                        attributes: serialize_attributes(attributes),
                        royalties,
                        name: name.unwrap_or_default().to_vec(),
                        hash: hash.map(|h| h.to_vec()),
                        uri: uri.map(|u| u.to_vec()),
                    },
                );

                let opt_contract_path = self.address_to_code_path.get(address);
                self.mandos_generator
                    .set_account(address, acc, opt_contract_path.cloned());
            },
            None => panic!(
                "set_nft_balance: Account {:?} does not exist",
                address_to_hex(address)
            ),
        }
    }

    pub fn set_esdt_local_roles(
        &mut self,
        address: &Address,
        token_id: &[u8],
        roles: &[EsdtLocalRole],
    ) {
        match self.b_mock.accounts.get_mut(address) {
            Some(acc) => {
                let mut roles_raw = Vec::new();
                for role in roles {
                    roles_raw.push(role.as_role_name().to_vec());
                }
                acc.esdt.set_roles(token_id.to_vec(), roles_raw);

                let opt_contract_path = self.address_to_code_path.get(address);
                self.mandos_generator
                    .set_account(address, acc, opt_contract_path.cloned());
            },
            None => panic!(
                "set_esdt_local_roles: Account {:?} does not exist",
                address_to_hex(address)
            ),
        }
    }

    pub fn set_block_epoch(&mut self, block_epoch: u64) {
        self.b_mock.current_block_info.block_epoch = block_epoch;

        self.mandos_generator.set_block_info(
            &self.b_mock.current_block_info,
            &self.b_mock.previous_block_info,
        );
    }

    pub fn set_block_nonce(&mut self, block_nonce: u64) {
        self.b_mock.current_block_info.block_nonce = block_nonce;

        self.mandos_generator.set_block_info(
            &self.b_mock.current_block_info,
            &self.b_mock.previous_block_info,
        );
    }

    pub fn set_block_random_seed(&mut self, block_random_seed: Box<[u8; 48]>) {
        self.b_mock.current_block_info.block_random_seed = block_random_seed;

        self.mandos_generator.set_block_info(
            &self.b_mock.current_block_info,
            &self.b_mock.previous_block_info,
        );
    }

    pub fn set_block_round(&mut self, block_round: u64) {
        self.b_mock.current_block_info.block_round = block_round;

        self.mandos_generator.set_block_info(
            &self.b_mock.current_block_info,
            &self.b_mock.previous_block_info,
        );
    }

    pub fn set_block_timestamp(&mut self, block_timestamp: u64) {
        self.b_mock.current_block_info.block_timestamp = block_timestamp;

        self.mandos_generator.set_block_info(
            &self.b_mock.current_block_info,
            &self.b_mock.previous_block_info,
        );
    }

    pub fn set_prev_block_epoch(&mut self, block_epoch: u64) {
        self.b_mock.previous_block_info.block_epoch = block_epoch;

        self.mandos_generator.set_block_info(
            &self.b_mock.current_block_info,
            &self.b_mock.previous_block_info,
        );
    }

    pub fn set_prev_block_nonce(&mut self, block_nonce: u64) {
        self.b_mock.previous_block_info.block_nonce = block_nonce;

        self.mandos_generator.set_block_info(
            &self.b_mock.current_block_info,
            &self.b_mock.previous_block_info,
        );
    }

    pub fn set_prev_block_random_seed(&mut self, block_random_seed: Box<[u8; 48]>) {
        self.b_mock.previous_block_info.block_random_seed = block_random_seed;

        self.mandos_generator.set_block_info(
            &self.b_mock.current_block_info,
            &self.b_mock.previous_block_info,
        );
    }

    pub fn set_prev_block_round(&mut self, block_round: u64) {
        self.b_mock.previous_block_info.block_round = block_round;

        self.mandos_generator.set_block_info(
            &self.b_mock.current_block_info,
            &self.b_mock.previous_block_info,
        );
    }

    pub fn set_prev_block_timestamp(&mut self, block_timestamp: u64) {
        self.b_mock.previous_block_info.block_timestamp = block_timestamp;

        self.mandos_generator.set_block_info(
            &self.b_mock.current_block_info,
            &self.b_mock.previous_block_info,
        );
    }

    pub fn add_mandos_sc_call(
        &mut self,
        sc_call: ScCallMandos,
        opt_expect: Option<TxExpectMandos>,
    ) {
        self.mandos_generator
            .create_tx(&sc_call, opt_expect.as_ref());
    }

    pub fn add_mandos_sc_query(
        &mut self,
        sc_query: ScQueryMandos,
        opt_expect: Option<TxExpectMandos>,
    ) {
        self.mandos_generator
            .create_query(&sc_query, opt_expect.as_ref());
    }
}

impl<CB, ContractObjBuilder> ContractObjWrapper<CB, ContractObjBuilder>
where
    CB: ContractBase<Api = DebugApi> + CallableContract<DebugApi> + 'static,
    ContractObjBuilder: 'static + Copy + Fn(DebugApi) -> CB,
{
    pub fn execute_tx<TxFn: FnOnce(CB) -> StateChange>(
        self,
        caller: &Address,
        sc_address: &Address,
        egld_payment: &num_bigint::BigUint,
        tx_fn: TxFn,
    ) -> Self {
        self.execute_tx_any(caller, sc_address, egld_payment, Vec::new(), tx_fn)
    }

    pub fn execute_esdt_transfer<TxFn: FnOnce(CB) -> StateChange>(
        self,
        caller: &Address,
        sc_address: &Address,
        token_id: &[u8],
        esdt_nonce: u64,
        esdt_amount: &num_bigint::BigUint,
        tx_fn: TxFn,
    ) -> Self {
        let esdt_transfer = vec![TxInputESDT {
            token_identifier: token_id.to_vec(),
            nonce: esdt_nonce,
            value: esdt_amount.clone(),
        }];
        self.execute_tx_any(caller, sc_address, &rust_biguint!(0), esdt_transfer, tx_fn)
    }

    pub fn execute_esdt_multi_transfer<TxFn: FnOnce(CB) -> StateChange>(
        self,
        caller: &Address,
        sc_address: &Address,
        esdt_transfers: &[TxInputESDT],
        tx_fn: TxFn,
    ) -> Self {
        self.execute_tx_any(
            caller,
            sc_address,
            &rust_biguint!(0),
            esdt_transfers.to_vec(),
            tx_fn,
        )
    }

    pub fn execute_query<TxFn: FnOnce(CB)>(self, sc_address: &Address, query_fn: TxFn) -> Self {
        self.execute_tx(sc_address, sc_address, &rust_biguint!(0), |sc| {
            query_fn(sc);
            StateChange::Revert
        })
    }

    // deduplicates code for execution
    fn execute_tx_any<TxFn: FnOnce(CB) -> StateChange>(
        self,
        caller: &Address,
        sc_address: &Address,
        egld_payment: &num_bigint::BigUint,
        esdt_payments: Vec<TxInputESDT>,
        tx_fn: TxFn,
    ) -> Self {
        let rc_b_mock = Rc::new(self.b_mock);
        let tx_cache = TxCache::new(rc_b_mock.clone());
        let rust_zero = rust_biguint!(0);

        if egld_payment > &rust_zero {
            tx_cache.subtract_egld_balance(caller, egld_payment);
            tx_cache.increase_egld_balance(sc_address, egld_payment);
        }

        for esdt in &esdt_payments {
            if esdt.value > rust_zero {
                let metadata = tx_cache.subtract_esdt_balance(
                    caller,
                    &esdt.token_identifier,
                    esdt.nonce,
                    &esdt.value,
                );
                tx_cache.increase_esdt_balance(
                    sc_address,
                    &esdt.token_identifier,
                    esdt.nonce,
                    &esdt.value,
                    metadata,
                );
            }
        }

        let tx_input = build_tx_input(caller, sc_address, egld_payment, esdt_payments);
        let tx_context_rc = Rc::new(TxContext::new(tx_input, tx_cache));
        TxContextStack::static_push(tx_context_rc.clone());

        let debug_api = DebugApi::new(tx_context_rc);
        let obj_builder = self.obj_builders.get(sc_address).unwrap();
        let sc = (obj_builder)(debug_api);

        let state_change = tx_fn(sc);

        let api_after_exec = Rc::try_unwrap(TxContextStack::static_pop()).unwrap();
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
            obj_builders: self.obj_builders,
            b_mock: new_b_mock,
            address_to_code_path: self.address_to_code_path,
            mandos_generator: self.mandos_generator,
            workspace_path: self.workspace_path,
            _phantom: PhantomData,
        }
    }
}

fn build_tx_input(
    caller: &Address,
    dest: &Address,
    egld_value: &num_bigint::BigUint,
    esdt_values: Vec<TxInputESDT>,
) -> TxInput {
    TxInput {
        from: caller.clone(),
        to: dest.clone(),
        egld_value: egld_value.clone(),
        esdt_values,
        func_name: Vec::new(),
        args: Vec::new(),
        gas_limit: u64::MAX,
        gas_price: 0,
        tx_hash: H256::zero(),
    }
}

fn address_to_hex(address: &Address) -> String {
    hex::encode(address.as_bytes())
}

fn serialize_attributes<T: elrond_wasm::elrond_codec::TopEncode>(attributes: &T) -> Vec<u8> {
    let mut serialized_attributes = Vec::new();
    if let Result::Err(err) = attributes.top_encode(&mut serialized_attributes) {
        panic!("Failed to encode attributes: {:?}", err)
    }

    serialized_attributes
}

fn convert_full_fn<CB, ContractObjBuilder>(
    func: ContractObjBuilder,
) -> Box<dyn Fn(DebugApi) -> Box<dyn CallableContract<DebugApi>>>
where
    CB: ContractBase<Api = DebugApi> + CallableContract<DebugApi> + 'static,
    ContractObjBuilder: 'static + Fn(DebugApi) -> CB,
{
    let raw_closure = move |context| convert_part(func(context));

    Box::new(raw_closure)
}

fn convert_part<CB>(c_base: CB) -> Box<dyn CallableContract<DebugApi>>
where
    CB: ContractBase<Api = DebugApi> + CallableContract<DebugApi> + 'static,
{
    Box::new(c_base)
}
