#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod deposit_info;
use deposit_info::DepositInfo;

pub const SECONDS_PER_ROUND: u64 = 6;

extern "C" {
    fn verifyEd25519(
        keyOffset: *const u8,
        messageOffset: *const u8,
        messageLength: i32,
        sigOffset: *const u8,
    ) -> i32;
}

#[elrond_wasm::contract]
pub trait DigitalCash {
    #[init]
    fn init(&self) {}

    //endpoints

    #[endpoint]
    #[payable("*")]
    fn fund(&self, address: ManagedAddress, valability: u64) {
        let payment: EsdtTokenPayment<Self::Api> = self.call_value().payment();
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
            b"successful withdrawal",
        );

        self.deposit(&address).clear();
    }

    #[endpoint]
    fn claim(&self, address: ManagedAddress, signature: ManagedBuffer) {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let deposit = self.deposit(&address).get();
        let caller_address = self.blockchain().get_caller();

        require!(
            deposit.expiration_round >= self.blockchain().get_block_round(),
            "deposit expired"
        );

        const HASH_DATA_BUFFER_LEN: usize = 1024;

        let sig_len = signature.len();
        require!(
            sig_len <= HASH_DATA_BUFFER_LEN,
            "Attributes too long, cannot copy into static buffer"
        );

        let mut signature_buffer = [0u8; HASH_DATA_BUFFER_LEN];

        let signature_buffer_slice = &mut signature_buffer[..sig_len];
        let load_result = signature.load_slice(0, signature_buffer_slice);
        require!(load_result.is_ok(), "Failed to load attributes into buffer");

        unsafe {
            let key = &address.to_byte_array()[..];
            let message = &caller_address.to_byte_array()[..];
            require!(
                verifyEd25519(
                    key.as_ptr(),
                    message.as_ptr(),
                    message.len() as i32,
                    signature_buffer_slice.as_ptr(),
                ) == 0,
                "invalid signature"
            );
        }

        self.send().direct(
            &caller_address,
            &deposit.token_name,
            deposit.nonce,
            &deposit.amount,
            b"successful claim",
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
