multiversx_sc::imports!();

/// Storage mapper test.
#[multiversx_sc::module]
pub trait SingleValueMapperLockedFeatures {
    #[view]
    #[storage_mapper("timelock_mapper")]
    fn timelock_mapper(&self) -> TimelockMapper<BigUint>;

    #[storage_mapper("timelock_mapper_with_key")]
    fn timelock_mapper_with_key(&self, extra_key: usize) -> TimelockMapper<ManagedBuffer>;

    #[storage_mapper_from_address("timelock_mapper_from_address")]
    fn timelock_mapper_from_address(
        &self,
        address: ManagedAddress,
    ) -> TimelockMapper<BigUint, ManagedAddress>;

    #[endpoint]
    fn timelock_set_initial_value(&self, initial_value: BigUint) {
        self.timelock_mapper().set(initial_value);
    }

    #[endpoint]
    fn timelock_set_unlock_timestamp(&self, unlock_timestamp: u64, future_value: BigUint) {
        let timelock = self.timelock_mapper();
        timelock.set_unlock_timestamp(unlock_timestamp, future_value);
    }

    #[endpoint]
    fn timelock_commit_action(&self) -> bool {
        let timelock = self.timelock_mapper();
        timelock.commit()
    }

    #[endpoint]
    fn timelock_get_unlock_timestamp(&self) -> u64 {
        let timelock = self.timelock_mapper();
        timelock.get_unlock_timestamp()
    }

    #[endpoint]
    fn timelock_get_future_value(&self) -> BigUint {
        let timelock = self.timelock_mapper();
        timelock.get_future_value()
    }

    #[endpoint]
    fn timelock_get_current_value_at_address(&self, address: ManagedAddress) -> BigUint {
        let timelock = self.timelock_mapper_from_address(address);
        timelock.get()
    }

    #[endpoint]
    fn timelock_get_unlock_timestamp_at_address(&self, address: ManagedAddress) -> u64 {
        let timelock = self.timelock_mapper_from_address(address);
        timelock.get_unlock_timestamp()
    }

    #[endpoint]
    fn timelock_get_future_value_at_address(&self, address: ManagedAddress) -> BigUint {
        let timelock = self.timelock_mapper_from_address(address);
        timelock.get_future_value()
    }
}
