elrond_wasm::imports!();

use super::storage;

#[elrond_wasm::module]
pub trait ForwarderRolesModule: storage::ForwarderStorageModule {
    #[endpoint(setLocalRoles)]
    fn set_local_roles(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        #[var_args] roles: VarArgs<EsdtLocalRole>,
    ) -> AsyncCall<Self::SendApi> {
        ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
            .set_special_roles(&address, &token_identifier, roles.as_slice())
            .async_call()
            .with_callback(self.callbacks().change_roles_callback())
    }

    #[endpoint(unsetLocalRoles)]
    fn unset_local_roles(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        #[var_args] roles: VarArgs<EsdtLocalRole>,
    ) -> AsyncCall<Self::SendApi> {
        ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
            .unset_special_roles(&address, &token_identifier, roles.as_slice())
            .async_call()
            .with_callback(self.callbacks().change_roles_callback())
    }

    #[callback]
    fn change_roles_callback(&self, #[call_result] result: AsyncCallResult<()>) {
        match result {
            AsyncCallResult::Ok(()) => {
                self.last_error_message().clear();
            },
            AsyncCallResult::Err(message) => {
                self.last_error_message().set(&message.err_msg);
            },
        }
    }
}
