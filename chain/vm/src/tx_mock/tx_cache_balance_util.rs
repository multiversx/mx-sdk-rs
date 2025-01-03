use num_bigint::BigUint;

use crate::{
    tx_execution::is_system_sc_address, tx_mock::TxPanic, types::VMAddress,
    world_mock::EsdtInstanceMetadata,
};

use super::TxCache;

impl TxCache {
    pub fn subtract_egld_balance(
        &self,
        address: &VMAddress,
        call_value: &BigUint,
    ) -> Result<(), TxPanic> {
        self.with_account_mut(address, |account| {
            if call_value > &account.egld_balance {
                return Err(TxPanic::vm_error("failed transfer (insufficient funds)"));
            }
            account.egld_balance -= call_value;
            Ok(())
        })
    }

    pub fn subtract_tx_gas(&self, address: &VMAddress, gas_limit: u64, gas_price: u64) {
        self.with_account_mut(address, |account| {
            let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
            assert!(
                account.egld_balance >= gas_cost,
                "Not enough balance to pay gas upfront"
            );
            account.egld_balance -= &gas_cost;
        });
    }

    pub fn increase_egld_balance(&self, address: &VMAddress, amount: &BigUint) {
        self.with_account_mut(address, |account| {
            account.egld_balance += amount;
        });
    }

    pub fn subtract_esdt_balance(
        &self,
        address: &VMAddress,
        esdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) -> Result<EsdtInstanceMetadata, TxPanic> {
        self.with_account_mut(address, |account| {
            let esdt_data_map = &mut account.esdt;
            let esdt_data = esdt_data_map
                .get_mut_by_identifier(esdt_token_identifier)
                .ok_or_else(err_insufficient_funds)?;

            let esdt_instances = &mut esdt_data.instances;
            let esdt_instance = esdt_instances
                .get_mut_by_nonce(nonce)
                .ok_or_else(err_insufficient_funds)?;

            let esdt_balance = &mut esdt_instance.balance;
            if &*esdt_balance < value {
                return Err(err_insufficient_funds());
            }

            *esdt_balance -= value;

            Ok(esdt_instance.metadata.clone())
        })
    }

    pub fn increase_esdt_balance(
        &self,
        address: &VMAddress,
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

    pub fn transfer_egld_balance(
        &self,
        from: &VMAddress,
        to: &VMAddress,
        value: &BigUint,
    ) -> Result<(), TxPanic> {
        if !is_system_sc_address(from) {
            self.subtract_egld_balance(from, value)?;
        }
        if !is_system_sc_address(to) {
            self.increase_egld_balance(to, value);
        }
        Ok(())
    }

    pub fn transfer_esdt_balance(
        &self,
        from: &VMAddress,
        to: &VMAddress,
        esdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) -> Result<(), TxPanic> {
        if !is_system_sc_address(from) && !is_system_sc_address(to) {
            let metadata = self.subtract_esdt_balance(from, esdt_token_identifier, nonce, value)?;
            self.increase_esdt_balance(to, esdt_token_identifier, nonce, value, metadata);
        }
        Ok(())
    }
}

fn err_insufficient_funds() -> TxPanic {
    TxPanic::vm_error("insufficient funds")
}
