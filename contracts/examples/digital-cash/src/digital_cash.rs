#![no_std]
#![allow(unused_attributes)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod deposit_info;

use deposit_info::{DepositInfo, FundType};

pub const SECONDS_PER_ROUND: u64 = 6;
pub use multiversx_sc::api::{ED25519_KEY_BYTE_LEN, ED25519_SIGNATURE_BYTE_LEN};

#[multiversx_sc::contract]
pub trait DigitalCash {
    #[init]
    fn init(&self) {}

    //endpoints

    #[endpoint]
    #[payable("*")]
    fn fund(&self, address: ManagedAddress, valability: u64) {
        let payment = self.call_value().egld_or_single_esdt();
        require!(
            payment.amount > BigUint::zero(),
            "amount must be greater than 0"
        );
        let fund_type = FundType {
            token: payment.token_identifier.clone(),
            nonce: payment.token_nonce,
        };

        let depositor_address = self.blockchain().get_caller();
        let mut deposit = DepositInfo {
            depositor_address,
            payment,
            expiration_round: self.get_expiration_round(valability),
        };

        if self.deposit(&address).contains_key(&fund_type) {
            self.deposit(&address).entry(fund_type).and_modify(|fund| {
                deposit.payment.amount += fund.payment.amount.clone();
                deposit.expiration_round = deposit.expiration_round.max(fund.expiration_round);
            });
        } else {
            self.deposit(&address).insert(fund_type, deposit);
        }
    }

    #[endpoint]
    fn withdraw(&self, address: ManagedAddress) {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let mut withdrawed_tokens = ManagedVec::<Self::Api, FundType<Self::Api>>::new();
        let block_round = self.blockchain().get_block_round();
        let mut transfer_occured = false;
        for (key, deposit) in self.deposit(&address).iter() {
            if deposit.expiration_round < block_round {
                self.send().direct(
                    &deposit.depositor_address,
                    &deposit.payment.token_identifier,
                    deposit.payment.token_nonce,
                    &deposit.payment.amount,
                );
                transfer_occured = true;
                withdrawed_tokens.push(key);
            }
        }

        require!(transfer_occured, "withdrawal has not been available yet");

        for token in withdrawed_tokens.iter() {
            self.deposit(&address).remove(&token);
        }
    }

    #[endpoint]
    fn claim(
        &self,
        address: ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let caller_address = self.blockchain().get_caller();

        let addr = address.as_managed_byte_array();
        let message = caller_address.as_managed_buffer();

        let mut withdrawed_tokens = ManagedVec::<Self::Api, FundType<Self::Api>>::new();
        let mut transfer_occured = false;
        let block_round = self.blockchain().get_block_round();
        require!(
            self.crypto()
                .verify_ed25519_legacy_managed::<32>(addr, message, &signature),
            "invalid signature"
        );

        for (key, deposit) in self.deposit(&address).iter() {
            if deposit.expiration_round >= block_round {
                self.send().direct(
                    &caller_address,
                    &deposit.payment.token_identifier,
                    deposit.payment.token_nonce,
                    &deposit.payment.amount,
                );
                transfer_occured = true;
                withdrawed_tokens.push(key);
            }
        }
        require!(transfer_occured, "deposit expired");

        for token in withdrawed_tokens.iter() {
            self.deposit(&address).remove(&token);
        }
    }

    #[endpoint]
    fn forward(&self, address: ManagedAddress, forward_address: ManagedAddress) {
        let caller = self.blockchain().get_caller();

        for (key, fund) in self.deposit(&address).iter() {
            require!(
                fund.depositor_address == caller,
                "only depositor can forward"
            );
            let forwarded_fund = DepositInfo {
                depositor_address: forward_address.clone(),
                payment: fund.payment,
                expiration_round: fund.expiration_round,
            };
            self.deposit(&address).insert(key, forwarded_fund);
        }
        self.deposit(&caller).clear();
    }

    //views

    #[view(amount)]
    fn get_amount(
        &self,
        address: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        nonce: u64,
    ) -> BigUint {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let data = self.deposit(&address).get(&FundType { token, nonce });
        let mut amount = BigUint::zero();
        if let Some(fund) = data {
            amount = fund.payment.amount;
        } else {
            require!(!self.deposit(&address).is_empty(), "non-existent key");
        }
        amount
    }

    //private functions

    fn get_expiration_round(&self, valability: u64) -> u64 {
        let valability_rounds = valability / SECONDS_PER_ROUND;
        self.blockchain().get_block_round() + valability_rounds
    }

    //storage

    #[view]
    #[storage_mapper("deposit")]
    fn deposit(
        &self,
        donor: &ManagedAddress,
    ) -> MapMapper<FundType<Self::Api>, DepositInfo<Self::Api>>;
}
