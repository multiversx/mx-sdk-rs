#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Auction<BigUint: BigUintApi> {
	pub payment_token: TokenIdentifier,
	pub payment_token_nonce: u64,
	pub creator_royalties_percentage: BigUint,
	pub min_bid: BigUint,
	pub max_bid: BigUint,
	pub deadline: u64,
	pub original_owner: Address,
	pub current_bid: BigUint,
	pub current_winner: Address,
}

#[elrond_wasm_derive::contract(EsdtNftMarketplaceImpl)]
pub trait EsdtNftMarketplace {
	#[init]
	fn init(&self, bid_cut_percentage: u64) {
		self.bid_cut_percentage()
			.set(&BigUint::from(bid_cut_percentage));
	}

	// endpoints - owner-only

	#[endpoint]
	fn claim(&self) -> SCResult<()> {
		let caller = self.get_caller();
		require!(
			caller == self.get_owner_address(),
			"Only owner may call this function"
		);

		let mut gas_left_before = self.get_gas_left();
		let mut max_gas_cost = 0u64;

		for (token_identifier, amount) in self.claimable_funds().iter() {
			// reserve double to try and prevent edge case bugs
			if gas_left_before < 2 * max_gas_cost {
				break;
			}

			self.send()
				.direct(&caller, &token_identifier, &amount, b"claim");

			self.claimable_funds().remove(&token_identifier);

			let gas_left_after = self.get_gas_left();
			let operation_gas_cost = gas_left_before - gas_left_after;
			if max_gas_cost < operation_gas_cost {
				max_gas_cost = operation_gas_cost;
			}

			gas_left_before = self.get_gas_left();
		}

		Ok(())
	}

	#[endpoint(setCutPercentage)]
	fn set_percentage_cut(&self, new_cut_percentage: u64) -> SCResult<()> {
		only_owner!(self, "Only owner may call this function!");
		require!(
			new_cut_percentage > 0 && new_cut_percentage < PERCENTAGE_TOTAL,
			"Invalid percentage value, should be between 0 and 10,000"
		);

		self.bid_cut_percentage()
			.set(&BigUint::from(new_cut_percentage));

		Ok(())
	}

	// storage

	#[storage_mapper("bidCutPerecentage")]
	fn bid_cut_percentage(&self) -> SingleValueMapper<Self::Storage, BigUint>;

	#[storage_mapper("claimableFunds")]
	fn claimable_funds(&self) -> MapMapper<Self::Storage, TokenIdentifier, BigUint>;

	#[storage_mapper("auctionForToken")]
	fn auction_for_token(
		&self,
		token_type: TokenIdentifier,
		token_nonce: u64,
	) -> SingleValueMapper<Self::Storage, Auction<BigUint>>;
}
