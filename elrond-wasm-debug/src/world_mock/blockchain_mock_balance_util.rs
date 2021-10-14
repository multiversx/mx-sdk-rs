use elrond_wasm::types::Address;
use num_bigint::BigUint;
use num_traits::Zero;

use crate::{address_hex, tx_mock::TxInputESDT};

use super::{BlockchainMock, BlockchainMockError};

impl BlockchainMock {
    pub fn subtract_egld_balance(
        &mut self,
        address: &Address,
        call_value: &BigUint,
    ) -> Result<(), BlockchainMockError> {
        let sender_account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account not found"));
        if &sender_account.egld_balance < call_value {
            return Err("failed transfer (insufficient funds)".into());
        }
        sender_account.egld_balance -= call_value;
        Ok(())
    }

    pub fn subtract_tx_gas(&mut self, address: &Address, gas_limit: u64, gas_price: u64) {
        let sender_account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account not found"));
        let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
        assert!(
            sender_account.egld_balance >= gas_cost,
            "Not enough balance to pay gas upfront"
        );
        sender_account.egld_balance -= &gas_cost;
    }

    pub fn increase_egld_balance(&mut self, address: &Address, amount: &BigUint) {
        let account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Receiver account not found"));
        account.egld_balance += amount;
    }

    pub fn subtract_esdt_balance(
        &mut self,
        address: &Address,
        esdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) {
        let sender_account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account {} not found", address_hex(address)));

        let esdt_data_map = &mut sender_account.esdt;
        let esdt_data = esdt_data_map
            .get_mut_by_identifier(esdt_token_identifier)
            .unwrap_or_else(|| {
                panic!(
                    "Account {} has no esdt tokens with name {}",
                    address_hex(address),
                    String::from_utf8(esdt_token_identifier.to_vec()).unwrap()
                )
            });

        let esdt_instances = &mut esdt_data.instances;
        let esdt_instance = esdt_instances.get_mut_by_nonce(nonce).unwrap_or_else(|| {
            panic!(
                "Esdt token {} has no nonce {}",
                String::from_utf8(esdt_token_identifier.to_vec()).unwrap(),
                nonce.to_string()
            )
        });
        let esdt_balance = &mut esdt_instance.balance;
        assert!(
            &*esdt_balance >= value,
            "Not enough esdt balance, have {}, need at least {}",
            esdt_balance,
            value
        );

        *esdt_balance -= value;
    }

    pub fn subtract_multi_esdt_balance(
        &mut self,
        address: &Address,
        esdt_transfers: &[TxInputESDT],
    ) {
        for esdt_transfer in esdt_transfers {
            if !esdt_transfer.value.is_zero() {
                self.subtract_esdt_balance(
                    address,
                    esdt_transfer.token_identifier.as_slice(),
                    esdt_transfer.nonce,
                    &esdt_transfer.value,
                );
            }
        }
    }

    pub fn increase_esdt_balance(
        &mut self,
        address: &Address,
        esdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) {
        let account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Receiver account not found"));

        if let Some(esdt_data) = account.esdt.get_mut_by_identifier(esdt_token_identifier) {
            esdt_data.instances.add(nonce, value.clone());
        } else {
            account
                .esdt
                .push_esdt(esdt_token_identifier.to_vec(), nonce, value.clone());
        }
    }

    pub fn increase_multi_esdt_balance(
        &mut self,
        address: &Address,
        esdt_transfers: &[TxInputESDT],
    ) {
        for esdt_transfer in esdt_transfers {
            if !esdt_transfer.value.is_zero() {
                self.increase_esdt_balance(
                    address,
                    esdt_transfer.token_identifier.as_slice(),
                    esdt_transfer.nonce,
                    &esdt_transfer.value,
                );
            }
        }
    }
}
