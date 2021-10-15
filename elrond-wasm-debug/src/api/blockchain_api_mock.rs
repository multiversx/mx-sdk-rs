use crate::DebugApi;
use elrond_wasm::types::{Address, BigUint, EsdtTokenData, ManagedAddress, TokenIdentifier, H256};

impl elrond_wasm::api::BlockchainApi for DebugApi {
    fn get_sc_address_legacy(&self) -> Address {
        self.input_ref().to.clone()
    }

    fn get_owner_address_legacy(&self) -> Address {
        self.with_contract_account(|account| {
            account
                .contract_owner
                .clone()
                .unwrap_or_else(|| panic!("contract owner address not set"))
        })
    }

    fn get_shard_of_address_legacy(&self, _address: &Address) -> u32 {
        panic!("get_shard_of_address not implemented")
    }

    fn is_smart_contract_legacy(&self, _address: &Address) -> bool {
        panic!("is_smart_contract not implemented")

        /*
        Mock used when testing the marketplace contract

        let mut addr_slice = [0u8; 32];
        hex::decode_to_slice(b"6d61726b6574706c6163655f636f6e74726163745f5f5f5f5f5f5f5f5f5f5f5f",
            &mut addr_slice);

        _address == &Address::from_slice(&addr_slice)
        */
    }

    fn get_caller_legacy(&self) -> Address {
        self.input_ref().from.clone()
    }

    fn get_balance_legacy(&self, address: &Address) -> BigUint<Self> {
        assert!(
            address == &self.get_sc_address_legacy(),
            "get balance not yet implemented for accounts other than the contract itself"
        );
        let egld_balance = self.with_contract_account(|account| account.egld_balance.clone());
        self.insert_new_big_uint(egld_balance)
    }

    fn get_state_root_hash_legacy(&self) -> H256 {
        panic!("get_state_root_hash_legacy not yet implemented")
    }

    fn get_tx_hash_legacy(&self) -> H256 {
        self.input_ref().tx_hash.clone()
    }

    fn get_gas_left(&self) -> u64 {
        self.input_ref().gas_limit
    }

    fn get_block_timestamp(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_timestamp
    }

    fn get_block_nonce(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_nonce
    }

    fn get_block_round(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_round
    }

    fn get_block_epoch(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_epoch
    }

    fn get_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        self.blockchain_ref()
            .current_block_info
            .block_random_seed
            .clone()
    }

    fn get_prev_block_timestamp(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_timestamp
    }

    fn get_prev_block_nonce(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_nonce
    }

    fn get_prev_block_round(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_round
    }

    fn get_prev_block_epoch(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_epoch
    }

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        self.blockchain_ref()
            .previous_block_info
            .block_random_seed
            .clone()
    }

    fn get_current_esdt_nft_nonce(
        &self,
        _address: &Address,
        _token: &TokenIdentifier<Self>,
    ) -> u64 {
        // TODO: Implement
        0u64
    }

    fn get_esdt_balance(
        &self,
        address: &ManagedAddress<Self>,
        token: &TokenIdentifier<Self>,
        nonce: u64,
    ) -> BigUint<Self> {
        assert!(
            address == &self.get_sc_address(),
            "get_esdt_balance not yet implemented for accounts other than the contract itself"
        );

        let esdt_balance = self.with_contract_account(|account| {
            account
                .esdt
                .get_esdt_balance(token.to_esdt_identifier().as_slice(), nonce)
        });
        self.insert_new_big_uint(esdt_balance)
    }

    fn get_esdt_token_data(
        &self,
        _address: &ManagedAddress<Self>,
        _token: &TokenIdentifier<Self>,
        _nonce: u64,
    ) -> EsdtTokenData<Self> {
        panic!("get_esdt_token_data not yet implemented")
    }
}
