elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::function_selector::FunctionSelector;
use crate::utils::{events, storage, structs::Token};

#[elrond_wasm_derive::module]
pub trait OwnerEndpointsModule: storage::StorageModule + events::EventsModule {
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

	/* 	#[endpoint]
	#[payable("*")]
	fn fund(
		&self,
		#[payment] payment: Self::BigUint,
		#[payment_token] token: TokenIdentifier,
	) -> SCResult<()> {
		require!(
			self.status() == Status::FundingPeriod,
			"cannot fund after deadline"
		);
		require!(token == self.cf_token_name().get(), "wrong token");

		let caller = self.blockchain().get_caller();
		self.deposit(&caller).update(|deposit| *deposit += payment);

		Ok(())
	}

	#[endpoint]
	fn claim(&self) -> SCResult<()> {
		let caller = self.blockchain().get_caller();
		require!(
			caller == self.blockchain().get_owner_address(),
			"only owner can claim successful funding"
		);

		let token_name = self.cf_token_name().get();
		let sc_balance = self.get_current_funds();

		self.send().direct(&caller, &token_name, &sc_balance, &[]);

		Ok(())
	}*/
}
