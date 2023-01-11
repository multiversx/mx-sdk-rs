multiversx_sc::imports!();

/// Storage mapper test.
#[multiversx_sc::module]
pub trait SingleValueMapperFeatures {
    #[view]
    #[storage_mapper("my_single_value_mapper")]
    fn map_my_single_value_mapper(&self) -> SingleValueMapper<BigInt>;

    #[endpoint]
    fn my_single_value_mapper_increment_1(&self, amount: BigInt) {
        let my_single_value_mapper = self.map_my_single_value_mapper();
        my_single_value_mapper.set(my_single_value_mapper.get() + amount);
    }

    /// Same as my_single_value_mapper_increment_1, but expressed more compactly.
    #[endpoint]
    fn my_single_value_mapper_increment_2(&self, amount: &BigInt) {
        self.map_my_single_value_mapper()
            .update(|value| *value += amount);
    }

    // Often times the update of a value is conditioned by a requirement
    // For example, when subtracting from a balance, we must first check that we have enough funds
    // The closure can return a Result, which can be propagated (either directly, or via sc_try!)
    #[endpoint]
    fn my_single_value_mapper_subtract_with_require(&self, amount: &BigInt) {
        self.map_my_single_value_mapper().update(|value| {
            require!(*value >= *amount, "not enough funds");
            *value -= amount;
        })
    }

    #[endpoint]
    fn my_single_value_mapper_set_if_empty(&self, value: BigInt) {
        self.map_my_single_value_mapper().set_if_empty(&value);
    }

    #[endpoint]
    fn clear_single_value_mapper(&self) {
        self.map_my_single_value_mapper().clear();
    }

    #[endpoint]
    fn get_from_address_single_value_mapper(&self) -> bool {
        self.map_my_single_value_mapper().is_empty()
    }

    #[endpoint]
    fn is_empty_single_value_mapper(&self) -> bool {
        self.map_my_single_value_mapper().is_empty()
    }

    #[endpoint]
    fn is_empty_at_address_single_value_mapper(&self, address: ManagedAddress) -> bool {
        self.map_my_single_value_mapper()
            .is_empty_at_address(&address)
    }

    #[endpoint]
    fn raw_byte_length_single_value_mapper(&self) -> usize {
        self.map_my_single_value_mapper().raw_byte_length()
    }
}
