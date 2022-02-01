#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod deposit_info;
use deposit_info::DepositInfo;

pub const SECONDS_PER_ROUND: u64 = 6;

#[elrond_wasm::contract]
pub trait DigitalCash {
    #[init]
    fn init(&self) {}

    //endpoints

    #[endpoint]
    #[payable("*")]
    fn fund(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: TokenIdentifier,
        address: ManagedAddress,
        valability: u64,
    ) -> SCResult<()> {
        require!(payment > BigUint::zero(), "amount must be greater than 0");
        require!(self.deposit(&address).is_empty(), "key already used");

        let nft_nonce = self.call_value().esdt_token_nonce();

        let deposit = &DepositInfo {
            amount: payment,
            depositor_address: self.blockchain().get_caller(),
            expiration_round: self.get_expiration_round(valability),
            token_name: token,
            nonce: nft_nonce,
        };

        self.deposit(&address).set(deposit);

        Ok(())
    }

    #[endpoint]
    fn withdraw(&self, address: ManagedAddress) -> SCResult<()> {
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

        Ok(())
    }

    #[endpoint]
    fn claim(&self, address: ManagedAddress, signature: ManagedBuffer) -> SCResult<()> {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let deposit = self.deposit(&address).get();
        let caller_address = self.blockchain().get_caller();

        require!(
            deposit.expiration_round >= self.blockchain().get_block_round(),
            "deposit expired"
        );
        require!(
            self.crypto().verify_ed25519(
                &address.to_byte_array()[..],
                &caller_address.to_byte_array()[..],
                signature.to_boxed_bytes().as_slice()
            ),
            "invalid signature"
        );

        self.send().direct(
            &caller_address,
            &deposit.token_name,
            deposit.nonce,
            &deposit.amount,
            b"successful claim",
        );
        self.deposit(&address).clear();

        Ok(())
    }

    //views

    #[view(amount)]
    fn get_amount(&self, address: ManagedAddress) -> SCResult<BigUint> {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let data = self.deposit(&address).get();

        Ok(data.amount)
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
