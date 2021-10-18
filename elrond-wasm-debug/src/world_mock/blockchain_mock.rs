use alloc::vec::Vec;
use elrond_wasm::types::Address;
use num_bigint::BigUint;
use num_traits::Zero;
use std::{collections::HashMap, rc::Rc};

use crate::{
    esdt_transfer_event_log,
    tx_mock::{SendBalance, TxCache, TxContext, TxContextRef, TxInput, TxLog},
    world_mock::AccountEsdt,
};

use super::{AccountData, BlockInfo, BlockchainMockError};

const ELROND_REWARD_KEY: &[u8] = b"ELRONDreward";

#[derive(Debug)]
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
    pub fn account_exists(&self, address: &Address) -> bool {
        self.accounts.contains_key(address)
    }

    pub fn commit_tx_cache(self: &mut Rc<Self>, tx_cache: TxCache) {
        let blockchain_updates = tx_cache.into_blockchain_updates();
        blockchain_updates.apply(Rc::get_mut(self).unwrap());
    }

    pub fn commit_tx(&mut self, tx_context: TxContextRef) {
        let blockchain_updates = tx_context.into_blockchain_updates();
        blockchain_updates.apply(self);
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

    // pub fn send_balance(
    //     &mut self,
    //     contract_address: &Address,
    //     send_balance_list: &[SendBalance],
    //     result_logs: &mut Vec<TxLog>,
    // ) -> Result<(), BlockchainMockError> {
    //     for send_balance in send_balance_list {
    //         if send_balance.token_identifier.is_empty() {
    //             self.subtract_egld_balance(contract_address, &send_balance.amount)?;
    //             self.increase_egld_balance(&send_balance.recipient, &send_balance.amount);
    //         } else {
    //             let esdt_token_identifier = send_balance.token_identifier.as_slice();
    //             let esdt_nonce = send_balance.nonce;
    //             self.subtract_esdt_balance(
    //                 contract_address,
    //                 esdt_token_identifier,
    //                 esdt_nonce,
    //                 &send_balance.amount,
    //             );
    //             self.increase_esdt_balance(
    //                 &send_balance.recipient,
    //                 esdt_token_identifier,
    //                 esdt_nonce,
    //                 &send_balance.amount,
    //             );

    //             let log = esdt_transfer_event_log(
    //                 contract_address.clone(),
    //                 send_balance.recipient.clone(),
    //                 esdt_token_identifier.to_vec(),
    //                 &send_balance.amount,
    //             );
    //             result_logs.insert(0, log); // TODO: it's a hack, should be inserted during execution, not here
    //         }
    //     }
    //     Ok(())
    // }

    // pub fn create_account_after_deploy(
    //     &mut self,
    //     tx_input: &TxInput,
    //     new_storage: HashMap<Vec<u8>, Vec<u8>>,
    //     contract_path: Vec<u8>,
    // ) -> Address {
    //     let sender = self
    //         .accounts
    //         .get(&tx_input.from)
    //         .unwrap_or_else(|| panic!("Unknown deployer"));
    //     let sender_nonce_before_tx = sender.nonce - 1;
    //     let new_address = self
    //         .get_new_address(tx_input.from.clone(), sender_nonce_before_tx)
    //         .unwrap_or_else(|| {
    //             panic!("Missing new address. Only explicit new deploy addresses supported")
    //         });

    //     let old_value = self.accounts.insert(
    //         new_address.clone(),
    //         AccountData {
    //             address: new_address.clone(),
    //             nonce: 0,
    //             egld_balance: tx_input.egld_value.clone(),
    //             storage: new_storage,
    //             esdt: AccountEsdt::default(),
    //             username: Vec::new(),
    //             contract_path: Some(contract_path),
    //             contract_owner: Some(tx_input.from.clone()),
    //         },
    //     );
    //     assert!(
    //         old_value.is_none(),
    //         "Account already exists at deploy address."
    //     );

    //     new_address
    // }

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
