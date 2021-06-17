elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{events, storage};

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Color {
	r: u8,
	g: u8,
	b: u8,
}
#[elrond_wasm_derive::module]

pub trait NFTModule<Color>: storage::StorageModule + events::EventsModule {
	#[payable("EGLD")]
	#[endpoint(nftIssue)]
	fn nft_issue(
		&self,
		#[payment] issue_cost: Self::BigUint,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
	) -> AsyncCall<Self::SendApi> {
		let caller = self.blockchain().get_caller();

		ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.issue_non_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				NonFungibleTokenProperties {
					can_freeze: true,
					can_wipe: true,
					can_pause: true,
					can_change_owner: true,
					can_upgrade: true,
					can_add_special_roles: true,
				},
			)
			.async_call()
			.with_callback(self.callbacks().nft_issue_callback(&caller))
	}

	#[callback]
	fn nft_issue_callback(
		&self,
		caller: &Address,
		#[call_result] result: AsyncCallResult<TokenIdentifier>,
	) {
		match result {
			AsyncCallResult::Ok(token_identifier) => {
				self.issued_token().set(&token_identifier);
			},
			AsyncCallResult::Err(_) => {
				let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();
				if token_identifier.is_egld() && returned_tokens > 0 {
					self.send().direct_egld(caller, &returned_tokens, &[]);
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
