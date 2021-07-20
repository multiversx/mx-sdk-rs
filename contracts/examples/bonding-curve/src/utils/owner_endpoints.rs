elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::function_selector::FunctionSelector;
use crate::utils::structs::{BondingCurve, TokenOwnershipData};
use crate::utils::{events, storage};

use super::structs::CurveArguments;

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
		identifier: TokenIdentifier,
		function: FunctionSelector<Self::BigUint>,
	) -> SCResult<()> {
		require!(
			!self.token_details(&identifier).is_empty(),
			"Token is not issued yet!"
		);
		self.bonding_curve(&identifier)
			.update(|bonding_curve| bonding_curve.curve = function);
		Ok(())
	}

	#[endpoint(deposit)]
	#[payable("*")]
	fn deposit(
		&self,
		#[payment] amount: Self::BigUint,
		#[payment_token] identifier: TokenIdentifier,
		#[payment_nonce] nonce: u64,
		#[var_args] payment_token: OptionalArg<TokenIdentifier>,
	) -> SCResult<()> {
		let caller = self.blockchain().get_caller();
		let mut set_payment = TokenIdentifier::egld();
		if self.bonding_curve(&identifier).is_empty() {
			set_payment = payment_token
				.into_option()
				.ok_or("Expected provided accepted_payment for the token")?;
		}
		if self.token_details(&identifier).is_empty() {
			self.token_details(&identifier).set(&TokenOwnershipData {
				token_nonces: [nonce].to_vec(),
				owner: caller.clone(),
			});
		} else {
			let details = self.token_details(&identifier).get();
			require!(
				details.owner == caller,
				"The token was already deposited by another address"
			);
			if !details.token_nonces.contains(&nonce) {
				self.token_details(&identifier)
					.update(|new_details| new_details.token_nonces.push(nonce));
			}
		}

		self.set_curve_storage(&identifier, amount.clone(), set_payment);
		self.owned_tokens(&caller).insert(identifier.clone());
		self.nonce_amount(&identifier, nonce)
			.update(|current_amount| *current_amount += amount);
		Ok(())
	}

	#[endpoint(claim)]
	fn claim(&self) -> SCResult<()> {
		let caller = self.blockchain().get_caller();
		require!(
			!self.owned_tokens(&caller).is_empty(),
			"You have nothing to claim"
		);
		for token in self.owned_tokens(&caller).iter() {
			let nonces = self.token_details(&token).get().token_nonces;
			for nonce in nonces {
				self.send().direct(
					&caller,
					&token,
					nonce,
					&self.nonce_amount(&token, nonce).get(),
					b"claim",
				);
				self.nonce_amount(&token, nonce).clear();
			}
			self.token_details(&token).clear();
		}
		self.owned_tokens(&caller).clear();

		Ok(())
	}

	fn set_curve_storage(
		&self,
		identifier: &TokenIdentifier,
		amount: Self::BigUint,
		payment: TokenIdentifier,
	) {
		let mut curve = FunctionSelector::None;
		let mut arguments;
		let payment_token;
		let payment_amount: Self::BigUint;

		if self.bonding_curve(identifier).is_empty() {
			arguments = CurveArguments {
				available_supply: amount.clone(),
				balance: amount,
			};
			payment_token = payment;
			payment_amount = 0u64.into();
		} else {
			let bonding_curve = self.bonding_curve(identifier).get();
			payment_token = bonding_curve.payment_token;
			payment_amount = bonding_curve.payment_amount;
			curve = bonding_curve.curve;
			arguments = bonding_curve.arguments;
			arguments.balance += &amount;
			arguments.available_supply += amount;
		}
		self.bonding_curve(identifier).set(&BondingCurve {
			curve,
			arguments,
			payment_token,
			payment_amount,
		});
	}
}
