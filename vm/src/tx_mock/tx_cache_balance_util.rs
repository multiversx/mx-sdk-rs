use crate::num_bigint::BigUint;
use multiversx_sc::types::heap::Address;

use crate::{tx_mock::TxPanic, world_mock::EsdtInstanceMetadata};

use super::TxCache;

impl TxCache {
    pub fn subtract_egld_balance(&self, address: &Address, call_value: &BigUint) {
        self.with_account_mut(address, |account| {
            if call_value > &account.egld_balance {
                std::panic::panic_any(TxPanic {
                    status: 10,
                    message: "failed transfer (insufficient funds)".to_string(),
                });
            }
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

    #[allow(clippy::redundant_closure)] // clippy is wrong here, `.unwrap_or_else(panic_insufficient_funds)` won't compile
    pub fn subtract_esdt_balance(
        &self,
        address: &Address,
        esdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) -> EsdtInstanceMetadata {
        self.with_account_mut(address, |account| {
            let esdt_data_map = &mut account.esdt;
            let esdt_data = esdt_data_map
                .get_mut_by_identifier(esdt_token_identifier)
                .unwrap_or_else(|| panic_insufficient_funds());

            let esdt_instances = &mut esdt_data.instances;
            let esdt_instance = esdt_instances
                .get_mut_by_nonce(nonce)
                .unwrap_or_else(|| panic_insufficient_funds());
            let esdt_balance = &mut esdt_instance.balance;
            if &*esdt_balance < value {
                panic_insufficient_funds();
            }

            *esdt_balance -= value;

            esdt_instance.metadata.clone()
        })
    }

    pub fn increase_esdt_balance(
        &self,
        address: &Address,
        esdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
        esdt_metadata: EsdtInstanceMetadata,
    ) {
        self.with_account_mut(address, |account| {
            account.esdt.increase_balance(
                esdt_token_identifier.to_vec(),
                nonce,
                value,
                esdt_metadata,
            );
        });
    }

    pub fn transfer_esdt_balance(
        &self,
        from: &Address,
        to: &Address,
        esdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) {
        let metadata = self.subtract_esdt_balance(from, esdt_token_identifier, nonce, value);

        self.increase_esdt_balance(to, esdt_token_identifier, nonce, value, metadata);
    }
}

fn panic_insufficient_funds() -> ! {
    std::panic::panic_any(TxPanic {
        status: 10,
        message: "insufficient funds".to_string(),
    });
}
