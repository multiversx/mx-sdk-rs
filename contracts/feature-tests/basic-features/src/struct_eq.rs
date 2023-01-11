multiversx_sc::imports!();

use crate::types::*;

/// Example of comparing structures in a contract.
#[multiversx_sc::module]
pub trait StructEquals {
    #[endpoint]
    fn managed_struct_eq(
        &self,
        s1: ExampleStructManaged<Self::Api>,
        s2: ExampleStructManaged<Self::Api>,
    ) -> bool {
        s1 == s2
    }
}
