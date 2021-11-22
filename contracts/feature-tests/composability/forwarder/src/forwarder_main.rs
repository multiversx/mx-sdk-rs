#![no_std]
#![allow(clippy::type_complexity)]

mod call_async;
mod call_sync;
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
        #[var_args] opt_data: OptionalArg<ManagedBuffer>,
    ) {
        let data = match opt_data {
            OptionalArg::Some(data) => data,
            OptionalArg::None => ManagedBuffer::new(),
        };
        self.send().direct_egld(to, amount, data);
    }

    #[endpoint]
    fn get_esdt_local_roles(
        &self,
        token_id: TokenIdentifier,
    ) -> ManagedMultiResultVec<ManagedBuffer> {
        let roles = self.blockchain().get_esdt_local_roles(&token_id);
        let mut result = ManagedMultiResultVec::new();
        for role in roles.iter_roles() {
            result.push(role.as_role_name().managed_into());
        }
        result
    }

    #[endpoint]
    fn check_token_has_roles(&self, token_id: TokenIdentifier, role: EsdtLocalRole) -> bool {
        let roles = self.blockchain().get_esdt_local_roles(&token_id);
        roles.has_role(&role)
    }
}
