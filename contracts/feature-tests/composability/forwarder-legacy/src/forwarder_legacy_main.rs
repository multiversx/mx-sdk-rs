#![no_std]
#![allow(clippy::type_complexity)]
#![allow(clippy::let_unit_value)]

pub mod call_async_legacy;
pub mod call_sync_legacy;
pub mod call_transf_exec_legacy;
pub mod contract_change_owner_legacy;
pub mod contract_deploy_legacy;
pub mod contract_upgrade_legacy;
pub mod esdt_legacy;
pub mod nft_legacy;
pub mod roles_legacy;
pub mod sft_legacy;
pub mod storage_legacy;

multiversx_sc::imports!();

/// Test contract for investigating backwards compatibility in smart contract calls.
#[multiversx_sc::contract]
pub trait ForwarderLegacy:
    call_sync_legacy::ForwarderSyncCallModule
    + call_async_legacy::ForwarderAsyncCallModule
    + call_transf_exec_legacy::ForwarderTransferExecuteModule
    + contract_change_owner_legacy::ChangeOwnerModule
    + contract_deploy_legacy::DeployContractModule
    + contract_upgrade_legacy::UpgradeContractModule
    + esdt_legacy::ForwarderEsdtModule
    + sft_legacy::ForwarderSftModule
    + nft_legacy::ForwarderNftModule
    + roles_legacy::ForwarderRolesModule
    + storage_legacy::ForwarderStorageModule
{
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn send_egld(&self, to: &ManagedAddress, amount: &BigUint) {
        self.send().direct_egld(to, amount);
    }
}
