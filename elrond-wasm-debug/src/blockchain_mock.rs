use super::mock_error::BlockchainMockError;
use crate::{
    contract_map::*, display_util::*, esdt_transfer_event_log, tx_context::*, SendBalance, TxInput,
    TxLog, TxOutput, TxPanic,
};
use alloc::{boxed::Box, vec::Vec};
use elrond_wasm::types::Address;
use num_bigint::BigUint;
use num_traits::Zero;
use std::{collections::HashMap, fmt, fmt::Write};

const ELROND_REWARD_KEY: &[u8] = b"ELRONDreward";
const SC_ADDRESS_NUM_LEADING_ZEROS: u8 = 8;

pub type AccountStorage = HashMap<Vec<u8>, Vec<u8>>;
pub type AccountEsdt = HashMap<Vec<u8>, BigUint>;

pub struct AccountData {
    pub address: Address,
    pub nonce: u64,
    pub balance: BigUint,
    pub storage: AccountStorage,
    pub esdt: AccountEsdt,
    pub username: Vec<u8>,
    pub contract_path: Option<Vec<u8>>,
    pub contract_owner: Option<Address>,
}

impl fmt::Display for AccountData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut esdt_buf = String::new();
        let mut esdt_keys: Vec<Vec<u8>> =
            self.esdt.clone().iter().map(|(k, _)| k.clone()).collect();
        esdt_keys.sort();

        for key in &esdt_keys {
            let value = self.esdt.get(key).unwrap();
            write!(
                &mut esdt_buf,
                "\n\t\t\t\t{} -> 0x{}",
                key_hex(key.as_slice()),
                hex::encode(value.to_bytes_be())
            )
            .unwrap();
        }

        let mut storage_buf = String::new();
        let mut storage_keys: Vec<Vec<u8>> = self.storage.iter().map(|(k, _)| k.clone()).collect();
        storage_keys.sort();

        for key in &storage_keys {
            let value = self.storage.get(key).unwrap();
            write!(
                &mut storage_buf,
                "\n\t\t\t{} -> 0x{}",
                key_hex(key.as_slice()),
                hex::encode(value.as_slice())
            )
            .unwrap();
        }

        write!(
            f,
            "AccountData {{
		nonce: {},
		balance: {},
		esdt: [{} ],
		username: {},
		storage: [{} ]
	}}",
            self.nonce,
            self.balance,
            esdt_buf,
            String::from_utf8(self.username.clone()).unwrap(),
            storage_buf
        )
    }
}

#[derive(Clone, Debug)]
pub struct BlockInfo {
    pub block_timestamp: u64,
    pub block_nonce: u64,
    pub block_round: u64,
    pub block_epoch: u64,
    pub block_random_seed: Box<[u8; 48]>,
}

impl BlockInfo {
    pub fn new() -> Self {
        BlockInfo {
            block_timestamp: 0,
            block_nonce: 0,
            block_round: 0,
            block_epoch: 0,
            block_random_seed: Box::from([0u8; 48]),
        }
    }
}

impl Default for BlockInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BlockchainMock {
    pub accounts: HashMap<Address, AccountData>,
    pub new_addresses: HashMap<(Address, u64), Address>,
    pub previous_block_info: BlockInfo,
    pub current_block_info: BlockInfo,
}

impl BlockchainMock {
    pub fn new() -> Self {
        BlockchainMock {
            accounts: HashMap::new(),
            new_addresses: HashMap::new(),
            previous_block_info: BlockInfo::new(),
            current_block_info: BlockInfo::new(),
        }
    }
}

impl Default for BlockchainMock {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockchainMock {
    pub fn add_account(&mut self, acct: AccountData) {
        self.accounts.insert(acct.address.clone(), acct);
    }

    pub fn validate_and_add_account(&mut self, acct: AccountData) {
        self.validate_account(&acct);
        self.add_account(acct);
    }

    pub fn print_accounts(&self) {
        let mut accounts_buf = String::new();
        for (address, account) in &self.accounts {
            write!(
                &mut accounts_buf,
                "\n\t{} -> {}",
                address_hex(address),
                account
            )
            .unwrap();
        }
        println!("Accounts: {}", &accounts_buf);
    }

    pub fn put_new_address(
        &mut self,
        creator_address: Address,
        creator_nonce: u64,
        new_address: Address,
    ) {
        self.new_addresses
            .insert((creator_address, creator_nonce), new_address);
    }

    fn get_new_address(&self, creator_address: Address, creator_nonce: u64) -> Option<Address> {
        self.new_addresses
            .get(&(creator_address, creator_nonce))
            .cloned()
    }

    pub fn validate_account(&self, account: &AccountData) {
        let is_sc = self.is_smart_contract_address(&account.address);
        let has_code = self.check_account_has_code(account);

        if is_sc && !has_code {
            panic!("Account has a smart contract address but no code");
        }

        if !is_sc && has_code {
            panic!("Account has no smart contract address but has code");
        }
    }

    pub fn is_smart_contract_address(&self, address: &Address) -> bool {
        address
            .as_bytes()
            .iter()
            .take(SC_ADDRESS_NUM_LEADING_ZEROS.into())
            .all(|item| item == &0u8)
    }

    pub fn check_account_has_code(&self, account: &AccountData) -> bool {
        !account
            .contract_path
            .as_ref()
            .unwrap_or(&Vec::<u8>::new())
            .is_empty()
    }

    pub fn subtract_tx_payment(
        &mut self,
        address: &Address,
        call_value: &BigUint,
    ) -> Result<(), BlockchainMockError> {
        let sender_account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account not found"));
        if &sender_account.balance < call_value {
            return Err("failed transfer (insufficient funds)".into());
        }
        sender_account.balance -= call_value;
        Ok(())
    }

    pub fn subtract_tx_gas(&mut self, address: &Address, gas_limit: u64, gas_price: u64) {
        let sender_account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account not found"));
        let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
        assert!(
            sender_account.balance >= gas_cost,
            "Not enough balance to pay gas upfront"
        );
        sender_account.balance -= &gas_cost;
    }

    pub fn increase_balance(&mut self, address: &Address, amount: &BigUint) {
        let account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Receiver account not found"));
        account.balance += amount;
    }

    pub fn send_balance(
        &mut self,
        contract_address: &Address,
        send_balance_list: &[SendBalance],
        result_logs: &mut Vec<TxLog>,
    ) -> Result<(), BlockchainMockError> {
        for send_balance in send_balance_list {
            if send_balance.token_name.is_empty() {
                self.subtract_tx_payment(contract_address, &send_balance.amount)?;
                self.increase_balance(&send_balance.recipient, &send_balance.amount);
            } else {
                let esdt_token_identifier = send_balance.token_name.as_slice();
                self.substract_esdt_balance(
                    contract_address,
                    esdt_token_identifier,
                    &send_balance.amount,
                );
                self.increase_esdt_balance(
                    &send_balance.recipient,
                    esdt_token_identifier,
                    &send_balance.amount,
                );

                let log = esdt_transfer_event_log(
                    contract_address.clone(),
                    send_balance.recipient.clone(),
                    esdt_token_identifier.to_vec(),
                    &send_balance.amount,
                );
                result_logs.insert(0, log); // TODO: it's a hack, should be inserted during execution, not here
            }
        }
        Ok(())
    }

    pub fn substract_esdt_balance(
        &mut self,
        address: &Address,
        esdt_token_identifier: &[u8],
        value: &BigUint,
    ) {
        let sender_account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account {} not found", address_hex(address)));

        let esdt_balance = sender_account
            .esdt
            .get_mut(esdt_token_identifier)
            .unwrap_or_else(|| {
                panic!(
                    "Account {} has no esdt tokens with name {}",
                    address_hex(address),
                    String::from_utf8(esdt_token_identifier.to_vec()).unwrap()
                )
            });

        assert!(
            *esdt_balance >= *value,
            "Not enough esdt balance, have {}, need at least {}",
            esdt_balance,
            value
        );

        *esdt_balance -= value;
    }

    pub fn increase_esdt_balance(
        &mut self,
        address: &Address,
        esdt_token_identifier: &[u8],
        value: &BigUint,
    ) {
        let account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Receiver account not found"));

        if account.esdt.contains_key(esdt_token_identifier) {
            let esdt_balance = account.esdt.get_mut(esdt_token_identifier).unwrap();
            *esdt_balance += value;
        } else {
            account
                .esdt
                .insert(esdt_token_identifier.to_vec(), value.clone());
        }
    }

    pub fn increase_nonce(&mut self, address: &Address) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.nonce += 1;
    }

    pub fn create_account_after_deploy(
        &mut self,
        tx_input: &TxInput,
        new_storage: HashMap<Vec<u8>, Vec<u8>>,
        contract_path: Vec<u8>,
    ) -> Address {
        let sender = self
            .accounts
            .get(&tx_input.from)
            .unwrap_or_else(|| panic!("Unknown deployer"));
        let sender_nonce_before_tx = sender.nonce - 1;
        let new_address = self
            .get_new_address(tx_input.from.clone(), sender_nonce_before_tx)
            .unwrap_or_else(|| {
                panic!("Missing new address. Only explicit new deploy addresses supported")
            });
        let mut esdt = HashMap::<Vec<u8>, BigUint>::new();
        if !tx_input.esdt_token_identifier.is_empty() {
            esdt.insert(
                tx_input.esdt_token_identifier.clone(),
                tx_input.esdt_value.clone(),
            );
        }

        let old_value = self.accounts.insert(
            new_address.clone(),
            AccountData {
                address: new_address.clone(),
                nonce: 0,
                balance: tx_input.call_value.clone(),
                storage: new_storage,
                esdt,
                username: Vec::new(),
                contract_path: Some(contract_path),
                contract_owner: Some(tx_input.from.clone()),
            },
        );
        if old_value.is_some() {
            panic!("Account already exists at deploy address.");
        }

        new_address
    }

    pub fn increase_validator_reward(&mut self, address: &Address, amount: &BigUint) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.balance += amount;
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

    pub fn try_set_username(&mut self, address: &Address, username: &[u8]) -> bool {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        if account.username.is_empty() {
            account.username = username.to_vec();
            true
        } else {
            false
        }
    }
}

pub fn execute_tx(
    tx_context: TxContext,
    contract_identifier: &[u8],
    contract_map: &ContractMap<TxContext>,
) -> TxOutput {
    let func_name = tx_context.tx_input_box.func_name.clone();
    let contract_inst = contract_map.new_contract_instance(contract_identifier, tx_context);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let call_successful = contract_inst.call(func_name.as_slice());
        if !call_successful {
            std::panic::panic_any(TxPanic {
                status: 1,
                message: b"invalid function (not found)".to_vec(),
            });
        }
        let context = contract_inst.into_api();
        context.into_output()
    }));
    match result {
        Ok(tx_output) => tx_output,
        Err(panic_any) => panic_result(panic_any),
    }
}

fn panic_result(panic_any: Box<dyn std::any::Any + std::marker::Send>) -> TxOutput {
    if panic_any.downcast_ref::<TxOutput>().is_some() {
        // async calls panic with the tx output directly
        // it is not a failure, simply a way to kill the execution
        return *panic_any.downcast::<TxOutput>().unwrap();
    }

    if let Some(panic_obj) = panic_any.downcast_ref::<TxPanic>() {
        return TxOutput::from_panic_obj(panic_obj);
    }

    if let Some(panic_string) = panic_any.downcast_ref::<String>() {
        return TxOutput::from_panic_string(panic_string.as_str());
    }

    TxOutput::from_panic_string("unknown panic")
}

/// Some data to get copied for the tx.
/// Would be nice maybe at some point to have a reference to the full blockchain mock in the tx context,
/// but for now, copying some data is enough.
#[derive(Clone, Debug)]
pub struct BlockchainTxInfo {
    pub previous_block_info: BlockInfo,
    pub current_block_info: BlockInfo,
    pub contract_balance: BigUint,
    pub contract_esdt: HashMap<Vec<u8>, BigUint>,
    pub contract_owner: Option<Address>,
}

impl BlockchainMock {
    pub fn create_tx_info(&self, contract_address: &Address) -> BlockchainTxInfo {
        if let Some(contract) = self.accounts.get(contract_address) {
            BlockchainTxInfo {
                previous_block_info: self.previous_block_info.clone(),
                current_block_info: self.current_block_info.clone(),
                contract_balance: contract.balance.clone(),
                contract_esdt: contract.esdt.clone(),
                contract_owner: contract.contract_owner.clone(),
            }
        } else {
            BlockchainTxInfo {
                previous_block_info: self.previous_block_info.clone(),
                current_block_info: self.current_block_info.clone(),
                contract_balance: 0u32.into(),
                contract_esdt: HashMap::new(),
                contract_owner: None,
            }
        }
    }
}
