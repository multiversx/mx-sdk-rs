#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use elrond_wasm_module_bonding_curve::utils::{events, owner_endpoints, storage, user_endpoints};

#[elrond_wasm::contract]
pub trait Contract:
    elrond_wasm_module_bonding_curve::BondingCurveModule
    + storage::StorageModule
    + events::EventsModule
    + user_endpoints::UserEndpointsModule
    + owner_endpoints::OwnerEndpointsModule
{
    #[init]
    fn init(&self) {}
}
