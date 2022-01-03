use crate::types::BigUint;

use super::ManagedTypeApi;

pub trait PrintApi: ManagedTypeApi {
    type PrintApiImpl: PrintApiImpl<ManagedTypeApi = Self>;

    fn print_api_impl() -> Self::PrintApiImpl;
}

pub trait PrintApiImpl {
    type ManagedTypeApi: ManagedTypeApi;

    fn print_biguint(&self, biguint: &BigUint<Self::ManagedTypeApi>);
}
