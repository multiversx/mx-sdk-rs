elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Color {
	r: u8,
	g: u8,
	b: u8,
}
use crate::{curve_arguments::SupplyType, events, storage};

#[elrond_wasm_derive::module]
pub trait CommonMethods {
	#[callback]
	fn nft_issue_callback(
		&self,
		caller: Address,
		#[call_result] result: AsyncCallResult<TokenIdentifier>,
	) {
		match result {
			AsyncCallResult::Ok(token_identifier) => {
				self.issued_token().set(&token_identifier);
			},
			AsyncCallResult::Err(_) => {
				let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();
				if token_identifier.is_egld() && returned_tokens > 0 {
					self.send().direct_egld(&caller, &returned_tokens, &[]);
				}
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
		color: Color,
		uri: BoxedBytes,
	) {
		self.send().esdt_nft_create::<Color>(
			&token_identifier,
			&amount,
			&name,
			&royalties,
			&hash,
			&color,
			&[uri],
		);
	}

	#[endpoint(nftBurn)]
	fn nft_burn(&self, token_identifier: TokenIdentifier, nonce: u64, amount: Self::BigUint) {
		self.send().esdt_nft_burn(&token_identifier, nonce, &amount);
	}
}
