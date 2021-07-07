elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::function_selector::FunctionSelector;
use crate::tokens::{common_methods, fungible_token, non_fungible_token, semi_fungible_token};
use crate::utils::{events, storage, structs::Token};

#[elrond_wasm_derive::module]
pub trait OwnerEndpointsModule:
	fungible_token::FungibleTokenModule
	+ non_fungible_token::NonFungibleTokenModule
	+ semi_fungible_token::SemiFungibleTokenModule
	+ storage::StorageModule
	+ events::EventsModule
	+ common_methods::CommonMethods
{
	#[endpoint(setLocalRoles)]
	fn set_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> AsyncCall<Self::SendApi> {
		ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.set_special_roles(&address, &token_identifier, roles.as_slice())
			.async_call()
			.with_callback(OwnerEndpointsModule::callbacks(self).change_roles_callback())
	}

	#[endpoint(unsetLocalRoles)]
	fn unset_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> AsyncCall<Self::SendApi> {
		ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.unset_special_roles(&address, &token_identifier, roles.as_slice())
			.async_call()
			.with_callback(OwnerEndpointsModule::callbacks(self).change_roles_callback())
	}

	#[callback]
	fn change_roles_callback(&self, #[call_result] result: AsyncCallResult<()>) -> SCResult<()> {
		match result {
			AsyncCallResult::Ok(()) => Ok(()),
			AsyncCallResult::Err(message) => Err(message.err_msg.into()),
		}
	}

	#[endpoint(setBondingCurve)]
	fn set_bonding_curve(
		&self,
		token: Token,
		function: FunctionSelector<Self::BigUint>,
	) -> SCResult<()> {
		require!(
			!self.bonding_curve(&token).is_empty(),
			"Token is not issued yet!"
		);
		self.bonding_curve(&token)
			.update(|bonding_curve| bonding_curve.curve = function);
		Ok(())
	}
}
