multiversx_sc::imports!();

/// Contains all events that can be emitted by the contract.
#[multiversx_sc::module]
pub trait BlockchainApiFeatures {
    #[endpoint]
    fn get_caller(&self) -> ManagedAddress {
        self.blockchain().get_caller()
    }

    #[endpoint]
    fn get_owner_address(&self) -> ManagedAddress {
        self.blockchain().get_owner_address()
    }

    #[endpoint]
    fn get_shard_of_address(&self, address: &ManagedAddress) -> u32 {
        self.blockchain().get_shard_of_address(address)
    }

    #[endpoint]
    fn is_smart_contract(&self, address: &ManagedAddress) -> bool {
        self.blockchain().is_smart_contract(address)
    }

    #[endpoint]
    fn get_state_root_hash(&self) -> ManagedByteArray<Self::Api, 32> {
        self.blockchain().get_state_root_hash()
    }

    #[endpoint]
    fn get_tx_hash(&self) -> ManagedByteArray<Self::Api, 32> {
        self.blockchain().get_tx_hash()
    }

    #[endpoint]
    fn get_gas_left(&self) -> u64 {
        self.blockchain().get_gas_left()
    }

    #[endpoint]
    fn get_cumulated_validator_rewards(&self) -> BigUint {
        self.blockchain().get_cumulated_validator_rewards()
    }
}
