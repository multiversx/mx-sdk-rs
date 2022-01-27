elrond_wasm::imports!();

use crate::types::*;

/// Example of comparing structures in a contract.
#[elrond_wasm::module]
pub trait StructEquals {
    #[endpoint]
    fn managed_struct_eq(
        &self,
        s1: ManagedSerExample<Self::Api>,
        s2: ManagedSerExample<Self::Api>,
    ) -> bool {
        s1 == s2
    }
}
