use crate::{api::PrintApi, types::BigUint};

impl PrintApi for super::UncallableApi {
    fn print_biguint(&self, _amount: &BigUint<Self>) {
        unreachable!();
    }
}
