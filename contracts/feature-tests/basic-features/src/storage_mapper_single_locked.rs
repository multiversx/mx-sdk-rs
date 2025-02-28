multiversx_sc::imports!();

/// Storage mapper test.
#[multiversx_sc::module]
pub trait SingleValueMapperLockedFeatures {
    #[view]
    #[storage_mapper("timelock_mapper")]
    fn timelock_mapper(&self) -> TimelockMapper<BigUint>;

    #[storage_mapper("timelock_mapper_with_key")]
    fn timelock_mapper_with_key(&self, extra_key: usize) -> TimelockMapper<ManagedBuffer>;

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
}
