multiversx_sc::imports!();

/// Storage mapper test.
#[multiversx_sc::module]
pub trait SingleValueMapperLockedFeatures {
    #[view]
    #[storage_mapper("single_value_mapper_with_timelock")]
    fn single_value_mapper_with_timelock(
        &self,
    ) -> SingleValueMapperWithTimelock<Self::Api, BigUint>;

    #[view]
    #[storage_mapper_with_timelock("svm_with_timelock_annotation")]
    fn svm_with_timelock_annotation(&self) -> SingleValueMapper<BigUint, CurrentStorageLocked>;

    #[storage_mapper_from_address("svm_with_timelock_from_address")]
    fn svm_with_timelock_from_address(
        &self,
        address: ManagedAddress,
    ) -> SingleValueMapper<BigUint, ManagedAddress>;

    #[storage_mapper("svm_with_timelock_and_key")]
    fn svm_with_timelock_and_key(
        &self,
        extra_key: usize,
    ) -> SingleValueMapperWithTimelock<Self::Api, ManagedBuffer>;

    #[endpoint]
    fn svm_with_timelock_set_unlock_timestamp(&self, unlock_timestamp: u64) {
        let mut svm = self.single_value_mapper_with_timelock();
        svm.set_unlock_timestamp(unlock_timestamp);
    }

    #[endpoint]
    fn svm_with_timelock_increment(&self, amount: BigUint) -> bool {
        let svm = self.single_value_mapper_with_timelock();
        svm.set_if_unlocked(svm.get() + amount)
    }

    #[endpoint]
    fn svm_with_timelock_annotation_increment(&self, amount: BigUint) -> bool {
        let svm = self.svm_with_timelock_annotation();
        svm.set_if_unlocked(svm.get() + amount)
    }

    #[endpoint]
    fn svm_with_timelock_update(&self, amount: &BigUint) {
        let svm = self.single_value_mapper_with_timelock();
        svm.update_if_unlocked(|value| *value += amount);
    }

    // Often times the update of a value is conditioned by a requirement
    // For example, when subtracting from a balance, we must first check that we have enough funds
    // The closure can return a Result, which can be propagated (either directly, or via sc_try!)
    #[endpoint]
    fn svm_with_timelock_subtract_with_require(&self, amount: &BigUint) {
        let svm = self.single_value_mapper_with_timelock();
        svm.update_if_unlocked(|value| {
            require!(*value >= *amount, "not enough funds");
            *value -= amount;
        })
    }

    #[endpoint]
    fn svm_with_timelock_clear(&self) -> bool {
        self.single_value_mapper_with_timelock().clear_if_unlocked()
    }

    #[endpoint]
    fn svm_with_timelock_get_unlock_timestamp(&self) -> u64 {
        self.single_value_mapper_with_timelock()
            .get_unlock_timestamp()
    }

    #[endpoint]
    fn svm_with_timelock_is_empty(&self) -> bool {
        self.single_value_mapper_with_timelock().is_empty()
    }

    #[endpoint]
    fn svm_with_timelock_raw_byte_len(&self) -> usize {
        self.single_value_mapper_with_timelock().raw_byte_length()
    }

    #[endpoint]
    fn svm_with_timelock_and_key_set(&self, key: usize, value: ManagedBuffer) -> bool {
        self.svm_with_timelock_and_key(key).set_if_unlocked(value)
    }

    #[endpoint]
    fn svm_with_timelock_is_empty_at_address(&self, address: ManagedAddress) -> bool {
        self.svm_with_timelock_from_address(address).is_empty()
    }

    #[endpoint]
    fn svm_with_timelock_get_from_address(&self, address: ManagedAddress) -> BigUint {
        self.svm_with_timelock_from_address(address).get()
    }

    #[endpoint]
    fn svm_with_timelock_get_unlock_timestamp_from_address(&self, address: ManagedAddress) -> u64 {
        self.svm_with_timelock_from_address(address)
            .get_unlock_timestamp()
    }
}
