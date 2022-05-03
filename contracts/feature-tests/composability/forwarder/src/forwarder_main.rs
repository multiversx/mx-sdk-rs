#![no_std]
#![allow(clippy::type_complexity)]

mod call_async;
pub mod call_sync;
mod call_transf_exec;
mod contract_change_owner;
mod contract_deploy;
mod contract_upgrade;
mod esdt;
mod nft;
mod roles;
mod sft;
mod storage;

elrond_wasm::imports!();

/// Test contract for investigating contract calls.
#[elrond_wasm::contract]
pub trait Forwarder:
    call_sync::ForwarderSyncCallModule
    + call_async::ForwarderAsyncCallModule
    + call_transf_exec::ForwarderTransferExecuteModule
    + contract_change_owner::ChangeOwnerModule
    + contract_deploy::DeployContractModule
    + contract_upgrade::UpgradeContractModule
    + esdt::ForwarderEsdtModule
    + sft::ForwarderSftModule
    + nft::ForwarderNftModule
    + roles::ForwarderRolesModule
    + storage::ForwarderStorageModule
{
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn send_egld(
        &self,
        to: &ManagedAddress,
        amount: &BigUint,
        opt_data: OptionalValue<ManagedBuffer>,
    ) {
        let data = match opt_data {
            OptionalValue::Some(data) => data,
            OptionalValue::None => ManagedBuffer::new(),
        };
        self.send().direct_egld(to, amount, data);
    }
}
