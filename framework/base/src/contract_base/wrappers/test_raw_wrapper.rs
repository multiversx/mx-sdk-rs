use core::marker::PhantomData;

use crate::{
    api::{TestApi, TestApiImpl},
    types::{
        BigUint, ManagedAddress, ManagedArgBuffer, ManagedBuffer, ManagedType, TokenIdentifier,
    },
};

#[derive(Default)]
pub struct TestRawWrapper<A>
where
    A: TestApi,
{
    _phantom: PhantomData<A>,
}

impl<A> TestRawWrapper<A>
where
    A: TestApi,
{
    pub fn new() -> Self {
        TestRawWrapper {
            _phantom: PhantomData,
        }
    }

    pub fn create_account(&self, address: &ManagedAddress<A>, nonce: u64, balance: &BigUint<A>) {
        A::test_api_impl().create_account(address.get_handle(), nonce, balance.get_handle());
    }

    pub fn register_new_address(
        self,
        owner: &ManagedAddress<A>,
        nonce: u64,
        new_address: &ManagedAddress<A>,
    ) {
        A::test_api_impl().register_new_address(
            owner.get_handle(),
            nonce,
            new_address.get_handle(),
        );
    }

    // Deploy a contract whose code was previously fetched using "fetchWasmSource" in Mandos.
    pub fn deploy_contract(
        self,
        owner: &ManagedAddress<A>,
        gas_limit: u64,
        value: &BigUint<A>,
        code_path: &ManagedBuffer<A>,
        arguments: &ManagedArgBuffer<A>,
    ) -> ManagedAddress<A> {
        let dest = ManagedAddress::zero();

        A::test_api_impl().deploy_contract(
            owner.get_handle(),
            gas_limit,
            value.get_handle(),
            code_path.get_handle(),
            arguments.get_handle(),
            dest.get_handle(),
        );

        dest
    }

    // Set storage of any account
    pub fn set_storage(
        self,
        address: &ManagedAddress<A>,
        key: &ManagedBuffer<A>,
        value: &ManagedBuffer<A>,
    ) {
        A::test_api_impl().set_storage(address.get_handle(), key.get_handle(), value.get_handle());
    }

    // Get storage of any account
    pub fn get_storage(
        self,
        address: &ManagedAddress<A>,
        key: &ManagedBuffer<A>,
    ) -> ManagedBuffer<A> {
        let dest = ManagedBuffer::new();

        A::test_api_impl().get_storage(address.get_handle(), key.get_handle(), dest.get_handle());

        dest
    }

    // Start a prank: set the caller address for contract calls until stop_prank
    pub fn start_prank(self, address: &ManagedAddress<A>) {
        A::test_api_impl().start_prank(address.get_handle());
    }

    // Stop a prank: reset the caller address
    pub fn stop_prank(self) {
        A::test_api_impl().stop_prank();
    }

    pub fn assume(self, p: bool) {
        A::test_api_impl().assume(p);
    }

    pub fn assert(self, p: bool) {
        A::test_api_impl().assert(p);
    }

    pub fn set_block_timestamp(self, timestamp: u64) {
        A::test_api_impl().set_block_timestamp(timestamp);
    }

    pub fn set_balance(self, address: &ManagedAddress<A>, value: &BigUint<A>) {
        A::test_api_impl().set_balance(address.get_handle(), value.get_handle());
    }

    pub fn set_esdt_balance(
        self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
        value: &BigUint<A>,
    ) {
        A::test_api_impl().set_esdt_balance(
            address.get_handle(),
            token_id.get_handle(),
            value.get_handle(),
        );
    }
}
