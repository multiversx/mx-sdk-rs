#![no_std]
#![allow(unused_attributes)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod constants;
mod deposit_info;
mod pay_fee_and_fund;
mod storage;

use constants::*;

pub use multiversx_sc::api::{ED25519_KEY_BYTE_LEN, ED25519_SIGNATURE_BYTE_LEN};

#[multiversx_sc::contract]
pub trait DigitalCash: pay_fee_and_fund::PayFeeAndFund + storage::StorageModule {
    #[init]
    fn init(&self, fee: BigUint, token: EgldOrEsdtTokenIdentifier) {
        self.fee().set(fee);
        self.fee_token().set(token);
    }

    #[endpoint]
    fn withdraw(&self, address: ManagedAddress) {
        let deposit_mapper = self.deposit(&address);
        let accepted_fee_token = self.fee_token().get();
        require!(!deposit_mapper.is_empty(), NON_EXISTENT_KEY_ERR_MSG);

        let block_round = self.blockchain().get_block_round();
        let deposit = deposit_mapper.take();
        require!(
            deposit.expiration_round < block_round,
            "withdrawal has not been available yet"
        );

        let mut egld_funds = deposit.egld_funds;
        let mut esdt_funds = deposit.esdt_funds;

        if accepted_fee_token == EgldOrEsdtTokenIdentifier::egld() {
            egld_funds += deposit.fees.value;
        } else {
            let esdt_fee_token = accepted_fee_token.unwrap_esdt();
            let esdt_fee = EsdtTokenPayment::new(esdt_fee_token, 0, deposit.fees.value);
            esdt_funds.push(esdt_fee);
        }

        if egld_funds > 0 {
            self.send()
                .direct_egld(&deposit.depositor_address, &egld_funds);
        }

        if !esdt_funds.is_empty() {
            self.send()
                .direct_multi(&deposit.depositor_address, &esdt_funds);
        }
    }

    #[endpoint]
    fn claim(
        &self,
        address: ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let deposit_mapper = self.deposit(&address);
        require!(!deposit_mapper.is_empty(), NON_EXISTENT_KEY_ERR_MSG);

        let caller_address = self.blockchain().get_caller();
        self.require_signature(&address, &caller_address, signature);

        let block_round = self.blockchain().get_block_round();
        let fee = self.fee().get();
        let mut deposit = deposit_mapper.take();
        require!(deposit.expiration_round >= block_round, "deposit expired");

        let num_tokens_transfered = deposit.get_num_tokens();
        let fee_cost = fee * num_tokens_transfered as u64;
        deposit.fees.value -= &fee_cost;

        self.collected_fees()
            .update(|collected_fees| *collected_fees += fee_cost);

        if deposit.egld_funds > 0 {
            self.send()
                .direct_egld(&caller_address, &deposit.egld_funds);
        }
        if !deposit.esdt_funds.is_empty() {
            self.send()
                .direct_multi(&caller_address, &deposit.esdt_funds);
        }
        if deposit.fees.value > 0 {
            self.send_fee_to_address(&deposit.fees.value, &deposit.depositor_address);
        }
    }

    #[endpoint(claimFees)]
    #[only_owner]
    fn claim_fees(&self) {
        let fees = self.collected_fees().take();
        if fees == 0 {
            return;
        }

        let caller_address = self.blockchain().get_caller();
        self.send_fee_to_address(&fees, &caller_address);
    }

    #[endpoint]
    fn forward(
        &self,
        address: ManagedAddress,
        forward_address: ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let deposit_mapper = self.deposit(&forward_address);
        require!(!deposit_mapper.is_empty(), CANNOT_DEPOSIT_FUNDS_ERR_MSG);

        let caller_address = self.blockchain().get_caller();
        let fee = self.fee().get();
        self.require_signature(&address, &caller_address, signature);

        let mut forwarded_deposit = self.deposit(&address).take();
        let num_tokens = forwarded_deposit.get_num_tokens();
        deposit_mapper.update(|deposit| {
            require!(
                deposit.egld_funds == BigUint::zero() && deposit.esdt_funds.is_empty(),
                "key already used"
            );
            require!(
                &fee * num_tokens as u64 <= deposit.fees.value,
                "cannot forward funds without the owner covering the fee cost first"
            );

            deposit.fees.num_token_to_transfer += num_tokens;
            deposit.valability = forwarded_deposit.valability;
            deposit.expiration_round = self.get_expiration_round(forwarded_deposit.valability);
            deposit.esdt_funds = forwarded_deposit.esdt_funds;
            deposit.egld_funds = forwarded_deposit.egld_funds;
        });

        let forward_fee = &fee * num_tokens as u64;
        forwarded_deposit.fees.value -= &forward_fee;

        self.collected_fees()
            .update(|collected_fees| *collected_fees += forward_fee);

        if forwarded_deposit.fees.value > 0 {
            self.send_fee_to_address(
                &forwarded_deposit.fees.value,
                &forwarded_deposit.depositor_address,
            );
        }
    }

    // views

    fn send_fee_to_address(&self, fee_amount: &BigUint, address: &ManagedAddress) {
        let accepted_fee_token = self.fee_token().get();
        if accepted_fee_token == EgldOrEsdtTokenIdentifier::egld() {
            self.send().direct_egld(address, fee_amount);
        } else {
            let esdt_fee_token = accepted_fee_token.unwrap_esdt();
            self.send()
                .direct_esdt(address, &esdt_fee_token, 0, fee_amount);
        }
    }

    #[view(getAmount)]
    fn get_amount(
        &self,
        address: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        nonce: u64,
    ) -> BigUint {
        let deposit_mapper = self.deposit(&address);
        require!(!deposit_mapper.is_empty(), NON_EXISTENT_KEY_ERR_MSG);

        let deposit = deposit_mapper.get();
        if token.is_egld() {
            return deposit.egld_funds;
        }

        for esdt in deposit.esdt_funds.into_iter() {
            if esdt.token_identifier == token && esdt.token_nonce == nonce {
                return esdt.amount;
            }
        }

        BigUint::zero()
    }

    fn require_signature(
        &self,
        address: &ManagedAddress,
        caller_address: &ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let addr = address.as_managed_buffer();
        let message = caller_address.as_managed_buffer();
        self.crypto()
            .verify_ed25519(addr, message, signature.as_managed_buffer());
    }

    // storage
}
