use crate::{
    api::CallValueApi,
    types::{BigUint, EsdtTokenType, TokenIdentifier},
};

use super::UncallableApi;

impl CallValueApi for UncallableApi {
    fn check_not_payable(&self) {
        unreachable!()
    }

    fn egld_value(&self) -> BigUint<Self> {
        unreachable!()
    }

    fn esdt_value(&self) -> BigUint<Self> {
        unreachable!()
    }

    fn token(&self) -> TokenIdentifier<Self> {
        unreachable!()
    }

    fn esdt_token_nonce(&self) -> u64 {
        unreachable!()
    }

    fn esdt_token_type(&self) -> EsdtTokenType {
        unreachable!()
    }

    fn esdt_num_transfers(&self) -> usize {
        unreachable!()
    }

    fn esdt_value_by_index(&self, _index: usize) -> BigUint<Self> {
        unreachable!()
    }

    fn token_by_index(&self, _index: usize) -> TokenIdentifier<Self> {
        unreachable!()
    }

    fn esdt_token_nonce_by_index(&self, _index: usize) -> u64 {
        unreachable!()
    }

    fn esdt_token_type_by_index(&self, _index: usize) -> EsdtTokenType {
        unreachable!()
    }
}
