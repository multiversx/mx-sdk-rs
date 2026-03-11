#![no_std]

mod fwd_blind_async_v1;
mod fwd_blind_async_v2;
mod fwd_blind_common;
mod fwd_blind_deploy;
mod fwd_blind_sync;
mod fwd_blind_transf_exec;
mod fwd_blind_upgrade;

multiversx_sc::imports!();

/// Contract that blindly forwards calls and payments, one endpoint per call type.
#[multiversx_sc::contract]
pub trait ForwarderBlind:
    fwd_blind_async_v1::ForwarderBlindAsyncV1
    + fwd_blind_async_v2::ForwarderBlindAsyncV2
    + fwd_blind_common::ForwarderBlindCommon
    + fwd_blind_deploy::ForwarderBlindDeploy
    + fwd_blind_sync::ForwarderBlindSync
    + fwd_blind_transf_exec::ForwarderBlindTransferExecute
    + fwd_blind_upgrade::ForwarderBlindUpgrade
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
