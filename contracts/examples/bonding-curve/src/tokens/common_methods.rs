elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
	events,
	function_selector::FunctionSelector,
	storage,
	utils::structs::{BondingCurve, CurveArguments, SupplyType, Token},
};

#[elrond_wasm_derive::module]
pub trait CommonMethods: storage::StorageModule + events::EventsModule {
	#[callback]
	fn nft_issue_callback(
		&self,
		caller: Address,
		#[call_result] result: AsyncCallResult<TokenIdentifier>,
	) {
		match result {
			AsyncCallResult::Ok(_) => {},
			AsyncCallResult::Err(message) => {
				let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();
				if token_identifier.is_egld() && returned_tokens > 0 {
					self.send().direct_egld(&caller, &returned_tokens, &[]);
				}
				self.last_error_message().set(&message.err_msg);
			},
		}
	}

	#[endpoint(nftCreate)]
	fn create(
		&self,
		identifier: TokenIdentifier,
		amount: Self::BigUint,
		name: BoxedBytes,
		royalties: Self::BigUint,
		hash: BoxedBytes,
		attributes: BoxedBytes,
		uri: BoxedBytes,
		#[var_args] max_supply: OptionalArg<Self::BigUint>,
		#[var_args] supply_type: OptionalArg<SupplyType>,
		#[var_args] payment: OptionalArg<TokenIdentifier>,
	) -> SCResult<()> {
		self.send().esdt_nft_create(
			&identifier,
			&amount,
			&name,
			&royalties,
			&hash,
			&attributes,
			&[uri],
		);
		let token;
		let curve = FunctionSelector::None;
		let mut arguments;
		let accepted_payment;

		if self.call_value().esdt_token_type() == EsdtTokenType::SemiFungible {
			token = Token {
				nonce: self.get_current_nonce(&identifier),
				identifier,
			};
			arguments = CurveArguments {
				supply_type: supply_type
					.into_option()
					.ok_or("Expected provided supply_type for new created token")?,
				max_supply: max_supply
					.into_option()
					.ok_or("Expected provided max_supply for new created token")?,
				available_supply: amount.clone(),
				balance: amount,
			};
			accepted_payment = payment
				.into_option()
				.ok_or("Expected provided accepted_payment for new created token")?;
		} else {
			token = Token {
				identifier,
				nonce: 0u64,
			};

			if self.bonding_curve(&token).is_empty() {
				arguments = CurveArguments {
					supply_type: supply_type
						.into_option()
						.ok_or("Expected provided supply_type for new created token")?,
					max_supply: max_supply
						.into_option()
						.ok_or("Expected provided max_supply for new created token")?,
					available_supply: amount.clone(),
					balance: amount,
				};

				accepted_payment = payment
					.into_option()
					.ok_or("Expected provided accepted_payment for new created token")?;
			} else {
				let bonding_curve = self.bonding_curve(&token).get();
				accepted_payment = bonding_curve.accepted_payment;
				arguments = bonding_curve.arguments;
				arguments.balance += amount.clone();
				arguments.available_supply += amount;
			}
		}
		self.bonding_curve(&token).set(&BondingCurve {
			curve,
			arguments,
			accepted_payment,
		});
		Ok(())
	}

	#[endpoint(nftBurn)]
	fn burn(&self, identifier: TokenIdentifier, nonce: u64, amount: Self::BigUint) -> SCResult<()> {
		self.send().esdt_nft_burn(&identifier, nonce, &amount);

		if self.call_value().esdt_token_type() == EsdtTokenType::SemiFungible {
			let token = &Token { identifier, nonce };

			if self.bonding_curve(token).is_empty() {
				return Err("Token has not been created.".into());
			}
			let mut bonding_curve = self.bonding_curve(token).get();
			bonding_curve.arguments.balance += &amount;
			bonding_curve.arguments.available_supply += &amount;
			self.bonding_curve(token).set(&bonding_curve);
		} else {
			let token = &Token {
				identifier,
				nonce: 0u64,
			};

			if self.bonding_curve(token).is_empty() {
				return Err("Token has not been created.".into());
			}
			self.bonding_curve(token).clear();
		}
		Ok(())
	}

	fn get_current_nonce(&self, identifier: &TokenIdentifier) -> u64 {
		self.blockchain()
			.get_current_esdt_nft_nonce(&self.blockchain().get_sc_address(), identifier)
	}
}
