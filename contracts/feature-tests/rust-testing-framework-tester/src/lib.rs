#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct NftDummyAttributes {
    pub creation_epoch: u64,
    pub cool_factor: u8,
}

#[elrond_wasm::derive::contract]
pub trait RustTestingFrameworkTester {
    #[init]
    fn init(&self) {
        self.total_value().set(&BigUint::from(1u32));
    }

    #[endpoint]
    fn sum(&self, first: BigUint, second: BigUint) -> BigUint {
        first + second
    }

    #[endpoint]
    fn sum_sc_result(&self, first: BigUint, second: BigUint) -> SCResult<BigUint> {
        require!(first > 0 && second > 0, "Non-zero required");
        Ok(first + second)
    }

    #[endpoint]
    fn get_caller_legacy(&self) -> Address {
        self.blockchain().get_caller_legacy()
    }

    #[endpoint]
    fn get_egld_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&TokenIdentifier::egld(), 0)
    }

    #[endpoint]
    fn get_esdt_balance(&self, token_id: TokenIdentifier, nonce: u64) -> BigUint {
        self.blockchain().get_sc_balance(&token_id, nonce)
    }

    #[payable("EGLD")]
    #[endpoint]
    fn receive_egld(&self) -> BigUint {
        self.call_value().egld_value()
    }

    #[payable("EGLD")]
    #[endpoint]
    fn recieve_egld_half(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = self.call_value().egld_value() / 2u32;
        self.send()
            .direct(&caller, &TokenIdentifier::egld(), 0, &payment_amount, &[]);
    }

    #[payable("*")]
    #[endpoint]
    fn receive_esdt(&self) -> (TokenIdentifier, BigUint) {
        let token_id = self.call_value().token();
        let amount = self.call_value().esdt_value();

        (token_id, amount)
    }

    #[payable("*")]
    #[endpoint]
    fn receive_esdt_half(&self) {
        let caller = self.blockchain().get_caller();
        let token_id = self.call_value().token();
        let amount = self.call_value().esdt_value() / 2u32;

        self.send().direct(&caller, &token_id, 0, &amount, &[]);
    }

    #[payable("*")]
    #[endpoint]
    fn receive_multi_esdt(&self) -> ManagedVec<EsdtTokenPayment<Self::Api>> {
        self.call_value().all_esdt_transfers()
    }

    #[payable("*")]
    #[endpoint]
    fn send_nft(
        &self,
        to: ManagedAddress,
        token_id: TokenIdentifier,
        nft_nonce: u64,
        amount: BigUint,
    ) {
        self.send().direct(&to, &token_id, nft_nonce, &amount, &[]);
    }

    #[endpoint]
    fn mint_esdt(&self, token_id: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send().esdt_local_mint(&token_id, nonce, &amount);
    }

    #[endpoint]
    fn burn_esdt(&self, token_id: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send().esdt_local_burn(&token_id, nonce, &amount);
    }

    #[endpoint]
    fn create_nft(
        &self,
        token_id: TokenIdentifier,
        amount: BigUint,
        attributes: NftDummyAttributes,
    ) -> u64 {
        self.send().esdt_nft_create(
            &token_id,
            &amount,
            &ManagedBuffer::new(),
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &attributes,
            &ManagedVec::new(),
        )
    }

    #[endpoint]
    fn get_block_epoch(&self) -> u64 {
        self.blockchain().get_block_epoch()
    }

    #[endpoint]
    fn get_block_nonce(&self) -> u64 {
        self.blockchain().get_block_nonce()
    }

    #[endpoint]
    fn get_block_timestamp(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    #[endpoint]
    fn add(&self, value: BigUint) {
        let caller = self.blockchain().get_caller();

        self.total_value().update(|val| *val += &value);
        self.value_per_caller(&caller).update(|val| *val += value);
    }

    #[storage_mapper("totalValue")]
    fn total_value(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("valuePerCaller")]
    fn value_per_caller(&self, caller: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
