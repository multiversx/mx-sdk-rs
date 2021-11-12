use crate::types::BigUint;

use super::ManagedTypeApi;

pub trait PrintApi: ManagedTypeApi {
    fn print_biguint(&self, biguint: &BigUint<Self>);
}
