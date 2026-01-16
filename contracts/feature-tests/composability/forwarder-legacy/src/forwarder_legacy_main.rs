#![no_std]
#![allow(clippy::type_complexity)]
#![allow(clippy::let_unit_value)]
#![allow(deprecated)]

pub mod fwd_call_async_legacy;
pub mod fwd_call_sync_legacy;
pub mod fwd_call_transf_exec_legacy;
pub mod fwd_change_owner_legacy;
pub mod fwd_deploy_legacy;
pub mod fwd_esdt_legacy;
pub mod fwd_fallible_legacy;
pub mod fwd_nft_legacy;
pub mod fwd_roles_legacy;
pub mod fwd_sft_legacy;
pub mod fwd_storage_legacy;
pub mod fwd_upgrade_legacy;

multiversx_sc::imports!();

/// Test contract for investigating backwards compatibility in smart contract calls.
#[multiversx_sc::contract]
pub trait ForwarderLegacy:
    fwd_call_sync_legacy::ForwarderSyncCallModule
    + fwd_call_async_legacy::ForwarderAsyncCallModule
    + fwd_call_transf_exec_legacy::ForwarderTransferExecuteModule
    + fwd_change_owner_legacy::ChangeOwnerModule
    + fwd_deploy_legacy::DeployContractModule
    + fwd_upgrade_legacy::UpgradeContractModule
    + fwd_esdt_legacy::ForwarderEsdtModule
    + fwd_fallible_legacy::ForwarderFallibleModule
    + fwd_sft_legacy::ForwarderSftModule
    + fwd_nft_legacy::ForwarderNftModule
    + fwd_roles_legacy::ForwarderRolesModule
    + fwd_storage_legacy::ForwarderStorageModule
{
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn send_egld(&self, to: &ManagedAddress, amount: &BigUint) {
        self.send().direct_egld(to, amount);
    }
}
