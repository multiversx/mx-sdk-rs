elrond_wasm::imports!();

use super::storage::*;

#[elrond_wasm_derive::module(ForwarderRolesModuleImpl)]
pub trait ForwarderRolesModule {
	#[module(ForwarderStorageModuleImpl)]
	fn storage_module(&self) -> ForwarderStorageModuleImpl<T, BigInt, BigUint>;

	#[endpoint(setLocalRoles)]
	fn set_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> AsyncCall<BigUint> {
		ESDTSystemSmartContractProxy::new()
			.set_special_roles(
				&address,
				token_identifier.as_esdt_identifier(),
				roles.as_slice(),
			)
			.async_call()
			.with_callback(self.callbacks().change_roles_callback())
	}

	#[endpoint(unsetLocalRoles)]
	fn unset_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> AsyncCall<BigUint> {
		ESDTSystemSmartContractProxy::new()
			.unset_special_roles(
				&address,
				token_identifier.as_esdt_identifier(),
				roles.as_slice(),
			)
			.async_call()
			.with_callback(self.callbacks().change_roles_callback())
	}

	#[callback]
	fn change_roles_callback(&self, #[call_result] result: AsyncCallResult<()>) {
		match result {
			AsyncCallResult::Ok(()) => {
				self.storage_module().last_error_message().clear();
			},
			AsyncCallResult::Err(message) => {
				self.storage_module()
					.last_error_message()
					.set(&message.err_msg);
			},
		}
	}
}
