use crate::{
    api::{PrintApi, PrintApiImpl},
    types::BigUint,
};

use super::UncallableApi;

impl PrintApi for UncallableApi {
    type PrintApiImpl = UncallableApi;

    fn print_api_impl() -> Self::PrintApiImpl {
        unreachable!()
    }
}

impl PrintApiImpl for UncallableApi {
    type ManagedTypeApi = UncallableApi;

    fn print_biguint(&self, _amount: &BigUint<Self::ManagedTypeApi>) {
        unreachable!();
    }
}
