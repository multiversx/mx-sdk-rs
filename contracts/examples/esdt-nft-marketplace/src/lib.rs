#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[derive(TopEncode, TopDecode, NestedEncode, TypeAbi)]
pub struct Auction<BigUint: BigUintApi> {
	pub payment_token: TokenIdentifier,
	pub payment_token_nonce: u64,
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

	// endpoints

	// TODO: Add macro-generated token-payment arguments once they're all available
	#[payable("*")]
	#[endpoint(auctionToken)]
	fn auction_token(
		&self,
		min_bid: BigUint,
		max_bid: BigUint,
		deadline: u64,
		accepted_payment_token: TokenIdentifier,
		#[var_args] opt_accepted_payment_token_nonce: OptionalArg<u64>,
	) -> SCResult<()> {
		let nft_type = self.call_value().token();
		let nft_nonce = self.call_value().esdt_token_nonce();

		require!(
			self.call_value().esdt_token_type() == EsdtTokenType::NonFungible,
			"Only Non-Fungible tokens can be auctioned"
		);
		require!(
			self.call_value().esdt_value() == BigUint::from(1u64),
			"Can only auction one token of a certain type at a time"
		);
		require!(
			!self.is_up_for_auction(&nft_type, nft_nonce),
			"There is already an auction for that token"
		);
		require!(
			min_bid > 0 && min_bid <= max_bid,
			"Min bid can't be 0 or higher than max bid"
		);
		require!(
			deadline > self.get_block_timestamp(),
			"Deadline can't be in the past"
		);

		let accepted_payment_nft_nonce = opt_accepted_payment_token_nonce
			.into_option()
			.unwrap_or_default();

		self.auction_for_token(&nft_type, nft_nonce).set(&Auction {
			payment_token: accepted_payment_token,
			payment_token_nonce: accepted_payment_nft_nonce,
			min_bid,
			max_bid,
			deadline,
			original_owner: self.get_caller(),
			current_bid: BigUint::zero(),
			current_winner: Address::zero(),
		});

		Ok(())
	}

	#[payable("*")]
	#[endpoint]
	fn bid(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> SCResult<()> {
		require!(
			self.is_up_for_auction(&nft_type, nft_nonce),
			"Token is not up for auction"
		);

		let (payment_amount, payment_token) = self.call_value().payment_token_pair();
		let payment_token_nonce = self.call_value().esdt_token_nonce();
		let caller = self.get_caller();
		let mut auction = self.auction_for_token(&nft_type, nft_nonce).get();

		require!(
			auction.original_owner != caller,
			"Can't bid on your own token"
		);
		require!(
			self.get_block_timestamp() < auction.deadline,
			"Auction ended already"
		);
		require!(
			payment_token == auction.payment_token
				&& payment_token_nonce == auction.payment_token_nonce,
			"Wrong token used as payment"
		);
		require!(auction.current_winner != caller, "Can't outbid yourself");
		require!(
			payment_amount >= auction.min_bid,
			"Bid must be higher than or equal to the min bid"
		);
		require!(
			payment_amount > auction.current_bid,
			"Bid must be higher than the current winning bid"
		);
		require!(
			payment_amount <= auction.max_bid,
			"Bid must be less than or equal to the max bid"
		);

		// refund losing bid
		if auction.current_winner != Address::zero() {
			self.send().direct_esdt_nft_via_transfer_exec(
				&auction.current_winner,
				&auction.payment_token.as_esdt_identifier(),
				auction.payment_token_nonce,
				&auction.current_bid,
				b"bid refund",
			);
		}

		// update auction bid and winner
		auction.current_bid = payment_amount;
		auction.current_winner = caller;
		self.auction_for_token(&nft_type, nft_nonce).set(&auction);

		Ok(())
	}

	// views

	#[view(isUpForAuction)]
	fn is_up_for_auction(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> bool {
		self.auction_for_token(nft_type, nft_nonce).is_empty()
	}

	#[view(getPaymentTokenForAuctionedNft)]
	fn get_payment_token_for_auctioned_nft(
		&self,
		nft_type: &TokenIdentifier,
		nft_nonce: u64,
	) -> Option<(TokenIdentifier, u64)> {
		if self.is_up_for_auction(nft_type, nft_nonce) {
			let auction = self.auction_for_token(nft_type, nft_nonce).get();

			Some((auction.payment_token, auction.payment_token_nonce))
		} else {
			None
		}
	}

	#[view(getMinMaxBid)]
	fn get_min_max_bid(
		&self,
		nft_type: &TokenIdentifier,
		nft_nonce: u64,
	) -> Option<(BigUint, BigUint)> {
		if self.is_up_for_auction(nft_type, nft_nonce) {
			let auction = self.auction_for_token(nft_type, nft_nonce).get();

			Some((auction.min_bid, auction.max_bid))
		} else {
			None
		}
	}

	#[view(getDeadline)]
	fn get_deadline(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> Option<u64> {
		if self.is_up_for_auction(nft_type, nft_nonce) {
			Some(self.auction_for_token(nft_type, nft_nonce).get().deadline)
		} else {
			None
		}
	}

	#[view(getOriginalOwner)]
	fn get_original_owner(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> Option<Address> {
		if self.is_up_for_auction(nft_type, nft_nonce) {
			Some(
				self.auction_for_token(nft_type, nft_nonce)
					.get()
					.original_owner,
			)
		} else {
			None
		}
	}

	#[view(getCurrentWinningBid)]
	fn get_current_winning_bid(
		&self,
		nft_type: &TokenIdentifier,
		nft_nonce: u64,
	) -> Option<BigUint> {
		if self.is_up_for_auction(nft_type, nft_nonce) {
			Some(
				self.auction_for_token(nft_type, nft_nonce)
					.get()
					.current_bid,
			)
		} else {
			None
		}
	}

	#[view(getCurrentWinner)]
	fn get_current_winner(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> Option<Address> {
		if self.is_up_for_auction(nft_type, nft_nonce) {
			Some(
				self.auction_for_token(nft_type, nft_nonce)
					.get()
					.current_winner,
			)
		} else {
			None
		}
	}

	#[view(getFullAuctionData)]
	fn get_full_auction_data(
		&self,
		nft_type: &TokenIdentifier,
		nft_nonce: u64,
	) -> Option<Auction<BigUint>> {
		if self.is_up_for_auction(nft_type, nft_nonce) {
			Some(self.auction_for_token(nft_type, nft_nonce).get())
		} else {
			None
		}
	}

	// storage

	#[storage_mapper("bidCutPerecentage")]
	fn bid_cut_percentage(&self) -> SingleValueMapper<Self::Storage, BigUint>;

	#[storage_mapper("claimableFunds")]
	fn claimable_funds(&self) -> MapMapper<Self::Storage, TokenIdentifier, BigUint>;

	#[storage_mapper("auctionForToken")]
	fn auction_for_token(
		&self,
		nft_type: &TokenIdentifier,
		nft_nonce: u64,
	) -> SingleValueMapper<Self::Storage, Auction<BigUint>>;
}
