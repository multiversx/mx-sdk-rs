elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
	function_selector::FunctionSelector,
	utils::{
		events, storage,
		structs::{BondingCurve, CurveArguments, SupplyType, Token},
	},
};

#[elrond_wasm_derive::module]
pub trait TokenMethods: storage::StorageModule + events::EventsModule {
	#[endpoint(ftLocalMint)]
	fn local_mint(&self, token_identifier: TokenIdentifier, amount: Self::BigUint) -> SCResult<()> {
		self.check_supply(
			&Token {
				identifier: token_identifier.clone(),
				nonce: 0u64,
			},
			&amount,
		)?;
		self.send().esdt_local_mint(&token_identifier, &amount);
		self.bonding_curve(&Token {
			identifier: token_identifier,
			nonce: 0u64,
		})
		.update(|bonding_curve| {
			bonding_curve.arguments.available_supply += &amount;
			bonding_curve.arguments.balance += &amount;
		});
		Ok(())
	}

	#[endpoint(ftLocalBurn)]
	fn local_burn(&self, token_identifier: TokenIdentifier, amount: Self::BigUint) {
		self.send().esdt_local_burn(&token_identifier, &amount);
		self.bonding_curve(&Token {
			identifier: token_identifier,
			nonce: 0u64,
		})
		.update(|bonding_curve| {
			bonding_curve.arguments.available_supply -= &amount;
			bonding_curve.arguments.balance -= &amount;
		});
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

		require!(!self.token_type(&identifier).is_empty(), "Token not issued");
		if self.token_type(&identifier).get() == EsdtTokenType::SemiFungible {
			token = Token {
				nonce: self.get_current_nonce(&identifier),
				identifier,
			};
			require!(token.nonce != 0, "Nonce should not be 0!");
			arguments = self.create_curve_arguments(supply_type, amount)?;
			accepted_payment = payment
				.into_option()
				.ok_or("Expected provided accepted_payment for new created token")?;
		} else {
			token = Token {
				identifier,
				nonce: 0u64,
			};

			if self.bonding_curve(&token).is_empty() {
				arguments = self.create_curve_arguments(supply_type, amount)?;
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

	#[endpoint(sftAddQuantity)]
	fn add_quantity(
		&self,
		identifier: TokenIdentifier,
		nonce: u64,
		amount: Self::BigUint,
	) -> SCResult<()> {
		let token = Token { identifier, nonce };
		self.check_supply(&token, &amount)?;
		self.send()
			.esdt_nft_add_quantity(&token.identifier, nonce, &amount);

		self.bonding_curve(&token).update(|bonding_curve| {
			bonding_curve.arguments.available_supply += &amount;
			bonding_curve.arguments.balance += amount;
		});
		Ok(())
	}

	fn create_curve_arguments(
		&self,
		#[var_args] supply_type: OptionalArg<SupplyType<Self::BigUint>>,
		amount: Self::BigUint,
	) -> SCResult<CurveArguments<Self::BigUint>> {
		Ok(CurveArguments {
			supply_type: supply_type
				.into_option()
				.ok_or("Expected provided supply_type for new created token")?,
			available_supply: amount.clone(),
			balance: amount,
		})
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
}
