#![no_std]
#![allow(clippy::type_complexity)]

mod common;
mod fwd_call_promise_direct;
mod fwd_call_promises;
mod fwd_call_promises_bt;
pub mod fwd_call_sync_bt;
pub mod promises_feature_proxy;
pub mod vault_proxy;

multiversx_sc::imports!();

/// Test contract for investigating the new async call framework.
#[multiversx_sc::contract]
pub trait PromisesFeatures:
    common::CommonModule
    + fwd_call_promises::CallPromisesModule
    + fwd_call_promise_direct::CallPromisesDirectModule
    + fwd_call_sync_bt::BackTransfersFeatureModule
    + fwd_call_promises_bt::CallPromisesBackTransfersModule
{
    #[init]
    fn init(&self) {}
}
