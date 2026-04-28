use multiversx_sc::imports::*;

use crate::constants::PERCENTAGE_TOTAL;

#[multiversx_sc::module]
pub trait UtilsModule {
    fn sum_array(&self, array: &ManagedVec<u8>) -> u32 {
        let mut sum = 0;

        for item in array {
            sum += item as u32;
        }

        sum
    }

    /// does not check if max - min >= amount, that is the caller's job
    fn get_distinct_random(&self, min: usize, max: usize) -> usize {
        let mut rand = RandomnessSource::new();
        rand.next_usize_in_range(min, max)
    }
    fn calculate_percentage_of(&self, value: &BigUint, percentage: &BigUint) -> BigUint {
        value * percentage / PERCENTAGE_TOTAL
    }
}
