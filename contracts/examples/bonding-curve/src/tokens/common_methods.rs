elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
	events,
	function_selector::{CurveArguments, FunctionSelector, SupplyType, Token},
	storage,
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
	fn nft_create(
		&self,
		token_identifier: TokenIdentifier,
		amount: Self::BigUint,
		name: BoxedBytes,
		royalties: Self::BigUint,
		hash: BoxedBytes,
		attributes: BoxedBytes,
		uri: BoxedBytes,
		max_supply: Self::BigUint,
		supply_type: SupplyType,
	) {
		self.send().esdt_nft_create(
			&token_identifier,
			&amount,
			&name,
			&royalties,
			&hash,
			&attributes,
			&[uri],
		);
		let token;
		let mut func = FunctionSelector::None;
		let mut args;
		if self.call_value().esdt_token_type() == EsdtTokenType::SemiFungible {
			token = Token {
				identifier: token_identifier.clone(),
				nonce: self.get_current_nonce(&token_identifier),
			};
			args = CurveArguments {
				supply_type,
				max_supply,
				available_supply: amount.clone(),
				balance: amount,
			};
		} else {
			token = Token {
				identifier: token_identifier.clone(),
				nonce: 0,
			};
			if self
				.bonding_curve(&Token {
					identifier: token_identifier.clone(),
					nonce: 0u64,
				})
				.is_empty()
			{
				args = CurveArguments {
					supply_type,
					max_supply,
					available_supply: amount.clone(),
					balance: amount.clone(),
				};
			} else {
				(func, args) = self
					.bonding_curve(&Token {
						identifier: token_identifier.clone(),
						nonce: 0u64,
					})
					.get();
				args.balance += &amount;
				args.available_supply += &amount;
			}
		}
		self.bonding_curve(&token).set(&(func, args));
	}

	#[endpoint(nftBurn)]
	fn nft_burn(
		&self,
		token_identifier: TokenIdentifier,
		nonce: u64,
		amount: Self::BigUint,
	) -> SCResult<()> {
		self.send().esdt_nft_burn(&token_identifier, nonce, &amount);

		if self
			.bonding_curve(&Token {
				identifier: token_identifier.clone(),
				nonce: 0u64,
			})
			.is_empty()
		{
			return Err("Token has not been created.".into());
		} else {
			if self.call_value().esdt_token_type() == EsdtTokenType::SemiFungible {
				let token = &Token {
					identifier: token_identifier.clone(),
					nonce,
				};

				let (func, mut args) = self.bonding_curve(&token).get();
				args.balance += &amount;
				args.available_supply += &amount;
				self.bonding_curve(&token).set(&(func, args));
			} else {
				self.bonding_curve(&Token {
					identifier: token_identifier.clone(),
					nonce: 0u64,
				})
				.clear();
			}
		}
		Ok(())
	}

	fn get_current_nonce(&self, identifier: &TokenIdentifier) -> u64 {
		self.blockchain()
			.get_current_esdt_nft_nonce(&self.blockchain().get_sc_address(), identifier)
	}
}
