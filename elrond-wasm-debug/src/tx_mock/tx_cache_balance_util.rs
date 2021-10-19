use elrond_wasm::types::Address;
use num_bigint::BigUint;
use num_traits::Zero;

use crate::{address_hex, tx_mock::TxInputESDT};

use super::TxCache;

impl TxCache {
    pub fn subtract_egld_balance(&self, address: &Address, call_value: &BigUint) {
        self.with_account_mut(address, |account| {
            assert!(
                &account.egld_balance >= call_value,
                "failed transfer (insufficient funds)"
            );
            account.egld_balance -= call_value;
        })
    }

    pub fn subtract_tx_gas(&self, address: &Address, gas_limit: u64, gas_price: u64) {
        self.with_account_mut(address, |account| {
            let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
            assert!(
                account.egld_balance >= gas_cost,
                "Not enough balance to pay gas upfront"
            );
            account.egld_balance -= &gas_cost;
        });
    }

    pub fn increase_egld_balance(&self, address: &Address, amount: &BigUint) {
        self.with_account_mut(address, |account| {
            account.egld_balance += amount;
        });
    }

    pub fn subtract_esdt_balance(
        &self,
        address: &Address,
        esdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) {
        self.with_account_mut(address, |account| {
            let esdt_data_map = &mut account.esdt;
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
        });
    }

    pub fn subtract_multi_esdt_balance(&self, address: &Address, esdt_transfers: &[TxInputESDT]) {
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
        &self,
        address: &Address,
        esdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) {
        self.with_account_mut(address, |account| {
            if let Some(esdt_data) = account.esdt.get_mut_by_identifier(esdt_token_identifier) {
                esdt_data.instances.add(nonce, value.clone());
            } else {
                account
                    .esdt
                    .push_esdt(esdt_token_identifier.to_vec(), nonce, value.clone());
            }
        });
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
