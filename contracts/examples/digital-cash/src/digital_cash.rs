#![no_std]
#![allow(unused_attributes)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod deposit_info;
use deposit_info::DepositInfo;

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
        require!(self.deposit(&address).is_empty(), "key already used");

        let deposit = DepositInfo {
            amount: payment.amount,
            depositor_address: self.blockchain().get_caller(),
            expiration_round: self.get_expiration_round(valability),
            token_name: payment.token_identifier,
            nonce: payment.token_nonce,
        };

        self.deposit(&address).set(&deposit);
    }

    #[endpoint]
    fn withdraw(&self, address: ManagedAddress) {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let deposit = self.deposit(&address).get();

        require!(
            deposit.expiration_round < self.blockchain().get_block_round(),
            "withdrawal has not been available yet"
        );
        self.send().direct(
            &deposit.depositor_address,
            &deposit.token_name,
            deposit.nonce,
            &deposit.amount,
        );

        self.deposit(&address).clear();
    }

    #[endpoint]
    fn claim(
        &self,
        address: ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let deposit = self.deposit(&address).get();
        let caller_address = self.blockchain().get_caller();

        require!(
            deposit.expiration_round >= self.blockchain().get_block_round(),
            "deposit expired"
        );

        let key = address.as_managed_byte_array();
        let message = caller_address.as_managed_buffer();
        require!(
            self.crypto()
                .verify_ed25519_legacy_managed::<32>(key, message, &signature),
            "invalid signature"
        );

        self.send().direct(
            &caller_address,
            &deposit.token_name,
            deposit.nonce,
            &deposit.amount,
        );
        self.deposit(&address).clear();
    }

    //views

    #[view(amount)]
    fn get_amount(&self, address: ManagedAddress) -> BigUint {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let data = self.deposit(&address).get();
        data.amount
    }

    //private functions

    fn get_expiration_round(&self, valability: u64) -> u64 {
        let valability_rounds = valability / SECONDS_PER_ROUND;
        self.blockchain().get_block_round() + valability_rounds
    }

    //storage

    #[view]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<DepositInfo<Self::Api>>;
}
