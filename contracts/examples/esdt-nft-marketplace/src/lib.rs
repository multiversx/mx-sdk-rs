#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%
const NFT_AMOUNT: u32 = 1; // Token has to be unique to be considered NFT

#[derive(TopEncode, TopDecode, NestedEncode, TypeAbi)]
pub struct Auction<BigUint: BigUintApi> {
	pub payment_token: EsdtToken,
	pub min_bid: BigUint,
	pub max_bid: BigUint,
	pub deadline: u64,
	pub original_owner: Address,
	pub current_bid: BigUint,
	pub current_winner: Address,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct EsdtToken {
	pub token_type: TokenIdentifier,
	pub nonce: u64,
}

#[elrond_wasm_derive::contract(EsdtNftMarketplaceImpl)]
pub trait EsdtNftMarketplace {
	#[init]
	fn init(&self, bid_cut_percentage: u64) {
		self.bid_cut_percentage()
			.set(&BigUint::from(bid_cut_percentage));
	}

	// endpoints - owner-only

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
			self.call_value().esdt_value() == BigUint::from(NFT_AMOUNT),
			"Token is not an NFT"
		);
		require!(
			!self.is_already_up_for_auction(&nft_type, nft_nonce),
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

		let accepted_payment_nft_nonce = if accepted_payment_token.is_egld() {
			0
		} else {
			opt_accepted_payment_token_nonce
				.into_option()
				.unwrap_or_default()
		};

		self.auction_for_token(&nft_type, nft_nonce).set(&Auction {
			payment_token: EsdtToken {
				token_type: accepted_payment_token,
				nonce: accepted_payment_nft_nonce,
			},
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
	fn bid(&self, nft_type: TokenIdentifier, nft_nonce: u64) -> SCResult<()> {
		require!(
			self.is_already_up_for_auction(&nft_type, nft_nonce),
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
			payment_token == auction.payment_token.token_type
				&& payment_token_nonce == auction.payment_token.nonce,
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
			self.transfer_esdt(
				&auction.current_winner,
				&auction.payment_token.token_type,
				auction.payment_token.nonce,
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

	#[endpoint(endAuction)]
	fn end_auction(&self, nft_type: TokenIdentifier, nft_nonce: u64) -> SCResult<()> {
		require!(
			self.is_already_up_for_auction(&nft_type, nft_nonce),
			"Token is not up for auction"
		);

		let auction = self.auction_for_token(&nft_type, nft_nonce).get();

		require!(
			self.get_block_timestamp() > auction.deadline || auction.current_bid == auction.max_bid,
			"Auction deadline has not passed nor is the current bid equal to max bid"
		);

		self.auction_for_token(&nft_type, nft_nonce).clear();

		if auction.current_winner != Address::zero() {
			let nft_info = self.get_esdt_token_data(
				&self.get_sc_address(),
				nft_type.as_esdt_identifier(),
				nft_nonce,
			);

			let percentage_cut = self.bid_cut_percentage().get();
			let creator_royalties =
				self.calculate_cut_amount(&auction.current_bid, &nft_info.royalties);

			// don't take anything if bid_cut would make it so the seller gets nothing
			let bid_cut_amount =
				if &percentage_cut + &nft_info.royalties < BigUint::from(PERCENTAGE_TOTAL) {
					self.calculate_cut_amount(&auction.current_bid, &percentage_cut)
				} else {
					BigUint::zero()
				};

			let seller_amount_to_send =
				&auction.current_bid - &creator_royalties - bid_cut_amount.clone();

			let token_id = &auction.payment_token.token_type;
			let nonce = auction.payment_token.nonce;

			if bid_cut_amount > BigUint::zero() {
				// send part as cut for contract owner
				let owner = self.get_owner_address();
				self.transfer_esdt(
					&owner,
					token_id,
					nonce,
					&bid_cut_amount,
					b"bid cut for sold token",
				);
			}

			// send part as royalties to creator
			self.transfer_esdt(
				&nft_info.creator,
				token_id,
				nonce,
				&creator_royalties,
				b"royalties for sold token",
			);

			// send rest of the bid to original owner
			self.transfer_esdt(
				&auction.original_owner,
				token_id,
				nonce,
				&seller_amount_to_send,
				b"sold token",
			);

			// send NFT to auction winner
			self.send().direct_esdt_nft_via_transfer_exec(
				&auction.current_winner,
				nft_type.as_esdt_identifier(),
				nft_nonce,
				&BigUint::from(NFT_AMOUNT),
				self.data_or_empty_if_sc(&auction.current_winner, b"bought token at auction"),
			);
		} else {
			// return to original owner
			self.send().direct_esdt_nft_via_transfer_exec(
				&auction.original_owner,
				nft_type.as_esdt_identifier(),
				nft_nonce,
				&BigUint::from(NFT_AMOUNT),
				self.data_or_empty_if_sc(&auction.original_owner, b"returned token"),
			);
		}

		Ok(())
	}

	#[endpoint]
	fn withdraw(&self, nft_type: TokenIdentifier, nft_nonce: u64) -> SCResult<()> {
		require!(
			self.is_already_up_for_auction(&nft_type, nft_nonce),
			"Token is not up for auction"
		);

		let auction = self.auction_for_token(&nft_type, nft_nonce).get();
		let caller = self.get_caller();

		require!(
			auction.original_owner == caller,
			"Only the original owner can withdraw"
		);
		require!(
			auction.current_bid == 0,
			"Can't withdraw, NFT already has bids"
		);

		self.auction_for_token(&nft_type, nft_nonce).clear();

		self.send().direct_esdt_nft_via_transfer_exec(
			&caller,
			nft_type.as_esdt_identifier(),
			nft_nonce,
			&BigUint::from(NFT_AMOUNT),
			self.data_or_empty_if_sc(&caller, b"returned token"),
		);

		Ok(())
	}

	// views

	#[view(isAlreadyUpForAuction)]
	fn is_already_up_for_auction(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> bool {
		!self.auction_for_token(nft_type, nft_nonce).is_empty()
	}

	#[view(getPaymentTokenForAuctionedNft)]
	fn get_payment_token_for_auctioned_nft(
		&self,
		nft_type: &TokenIdentifier,
		nft_nonce: u64,
	) -> Option<EsdtToken> {
		if self.is_already_up_for_auction(nft_type, nft_nonce) {
			Some(
				self.auction_for_token(nft_type, nft_nonce)
					.get()
					.payment_token,
			)
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
		if self.is_already_up_for_auction(nft_type, nft_nonce) {
			let auction = self.auction_for_token(nft_type, nft_nonce).get();

			Some((auction.min_bid, auction.max_bid))
		} else {
			None
		}
	}

	#[view(getDeadline)]
	fn get_deadline(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> Option<u64> {
		if self.is_already_up_for_auction(nft_type, nft_nonce) {
			Some(self.auction_for_token(nft_type, nft_nonce).get().deadline)
		} else {
			None
		}
	}

	#[view(getOriginalOwner)]
	fn get_original_owner(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> Option<Address> {
		if self.is_already_up_for_auction(nft_type, nft_nonce) {
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
		if self.is_already_up_for_auction(nft_type, nft_nonce) {
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
		if self.is_already_up_for_auction(nft_type, nft_nonce) {
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
		if self.is_already_up_for_auction(nft_type, nft_nonce) {
			Some(self.auction_for_token(nft_type, nft_nonce).get())
		} else {
			None
		}
	}

	// private

	fn calculate_cut_amount(&self, total_amount: &BigUint, cut_percentage: &BigUint) -> BigUint {
		total_amount * cut_percentage / BigUint::from(PERCENTAGE_TOTAL)
	}

	fn transfer_esdt(
		&self,
		to: &Address,
		token_id: &TokenIdentifier,
		nonce: u64,
		amount: &BigUint,
		data: &'static [u8],
	) {
		// nonce 0 means fungible ESDT or EGLD
		if nonce == 0 {
			self.send()
				.direct(to, &token_id, amount, self.data_or_empty_if_sc(to, data));
		} else {
			self.send().direct_esdt_nft_via_transfer_exec(
				to,
				token_id.as_esdt_identifier(),
				nonce,
				amount,
				self.data_or_empty_if_sc(to, data),
			);
		}
	}

	fn data_or_empty_if_sc(&self, dest: &Address, data: &'static [u8]) -> &[u8] {
		if self.is_smart_contract(dest) {
			&[]
		} else {
			data
		}
	}

	// storage

	#[storage_mapper("bidCutPerecentage")]
	fn bid_cut_percentage(&self) -> SingleValueMapper<Self::Storage, BigUint>;

	#[storage_mapper("auctionForToken")]
	fn auction_for_token(
		&self,
		nft_type: &TokenIdentifier,
		nft_nonce: u64,
	) -> SingleValueMapper<Self::Storage, Auction<BigUint>>;
}
