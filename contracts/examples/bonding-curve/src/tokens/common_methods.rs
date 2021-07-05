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
		token_type: EsdtTokenType,
		#[payment_token] token_identifier: TokenIdentifier,
		#[call_result] result: AsyncCallResult<TokenIdentifier>,
	) -> SCResult<()> {
		match result {
			AsyncCallResult::Ok(_) => {
				self.token_type(&token_identifier).set(&token_type);
				Ok(())
			},
			AsyncCallResult::Err(message) => {
				let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();
				if token_identifier.is_egld() && returned_tokens > 0 {
					self.send().direct_egld(&caller, &returned_tokens, &[]);
				}
				Err(message.err_msg.into())
			},
		}
	}

	#[callback]
	#[allow(clippy::too_many_arguments)]
	fn ft_issue_callback(
		&self,
		caller: Address,
		initial_supply: Self::BigUint,
		supply_type: SupplyType<Self::BigUint>,
		accepted_payment: TokenIdentifier,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] amount: Self::BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) -> SCResult<()> {
		match result {
			AsyncCallResult::Ok(()) => {
				self.bonding_curve(&Token {
					identifier: token_identifier.clone(),
					nonce: 0u64,
				})
				.set(&BondingCurve {
					curve: FunctionSelector::None,
					arguments: CurveArguments {
						supply_type,
						available_supply: initial_supply.clone(),
						balance: initial_supply,
					},
					accepted_payment,
				});
				self.token_type(&token_identifier)
					.set(&EsdtTokenType::Fungible);
				Ok(())
			},
			AsyncCallResult::Err(message) => {
				if token_identifier.is_egld() && amount > 0 {
					self.send().direct_egld(&caller, &amount, &[]);
				}

				Err(message.err_msg.into())
			},
		}
	}

	#[endpoint(nftCreate)]
	#[allow(clippy::too_many_arguments)]
	fn create(
		&self,
		identifier: TokenIdentifier,
		amount: Self::BigUint,
		name: BoxedBytes,
		royalties: Self::BigUint,
		hash: BoxedBytes,
		attributes: BoxedBytes,
		uri: BoxedBytes,
		#[var_args] supply_type: OptionalArg<SupplyType<Self::BigUint>>,
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
		let mut curve = FunctionSelector::None;
		let mut arguments;
		let accepted_payment;

		if self.token_type(&identifier).get() == EsdtTokenType::SemiFungible {
			token = Token {
				nonce: self.get_current_nonce(&identifier),
				identifier,
			};
			require!(token.nonce != 0, "Nonce should not be 0!");
			arguments = CurveArguments {
				supply_type: supply_type
					.into_option()
					.ok_or("Expected provided supply_type for new created token")?,
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
					available_supply: amount.clone(),
					balance: amount,
				};

				accepted_payment = payment
					.into_option()
					.ok_or("Expected provided accepted_payment for new created token")?;
			} else {
				self.check_supply(&token, &amount)?;
				let bonding_curve = self.bonding_curve(&token).get();
				accepted_payment = bonding_curve.accepted_payment;
				curve = bonding_curve.curve;
				arguments = bonding_curve.arguments;
				arguments.balance += &amount;
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

	fn check_supply(&self, token: &Token, amount: &Self::BigUint) -> SCResult<()> {
		let bonding_curve = self.bonding_curve(token).get();

		if bonding_curve.arguments.supply_type != SupplyType::Unlimited {
			require!(
				bonding_curve.arguments.available_supply
					< bonding_curve.arguments.supply_type.get_limit()?,
				"Maximum supply limit reached!"
			);

			require!(
				bonding_curve.arguments.available_supply + amount.clone()
					<= bonding_curve.arguments.supply_type.get_limit()?,
				"Minting will exceed the maximum supply limit!"
			);
		}
		Ok(())
	}

	#[callback]
	fn mint_callback(
		&self,
		token_identifier: TokenIdentifier,
		amount: &Self::BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) -> SCResult<()> {
		match result {
			AsyncCallResult::Ok(()) => {
				self.bonding_curve(&Token {
					identifier: token_identifier,
					nonce: 0u64,
				})
				.update(|bonding_curve| {
					bonding_curve.arguments.available_supply += amount;
					bonding_curve.arguments.balance += amount;
				});
				Ok(())
			},
			AsyncCallResult::Err(message) => Err(message.err_msg.into()),
		}
	}
}
