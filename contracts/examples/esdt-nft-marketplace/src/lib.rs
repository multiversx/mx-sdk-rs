#![no_std]

elrond_wasm::imports!();

pub mod auction;
use auction::*;

mod storage;
mod views;

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%
const NFT_AMOUNT: u32 = 1; // Token has to be unique to be considered NFT

pub struct BidSplitAmounts<BigUint: BigUintApi> {
	creator: BigUint,
	marketplace: BigUint,
	seller: BigUint,
}

#[elrond_wasm_derive::contract]
pub trait EsdtNftMarketplace: storage::StorageModule + views::ViewsModule {
	#[init]
	fn init(&self, bid_cut_percentage: u64) -> SCResult<()> {
		self.try_set_bid_cut_percentage(bid_cut_percentage)
	}

	// endpoints - owner-only

	#[endpoint(setCutPercentage)]
	fn set_percentage_cut(&self, new_cut_percentage: u64) -> SCResult<()> {
		only_owner!(self, "Only owner may call this function!");
		self.try_set_bid_cut_percentage(new_cut_percentage)
	}

	// endpoints

	#[payable("*")]
	#[endpoint(auctionToken)]
	fn auction_token(
		&self,
		#[payment_token] nft_type: TokenIdentifier,
		#[payment_nonce] nft_nonce: u64,
		#[payment_amount] nft_amount: Self::BigUint,
		min_bid: Self::BigUint,
		max_bid: Self::BigUint,
		deadline: u64,
		accepted_payment_token: TokenIdentifier,
		#[var_args] opt_accepted_payment_token_nonce: OptionalArg<u64>,
		#[var_args] opt_sft_max_one_per_user: OptionalArg<bool>,
		#[var_args] opt_start_time: OptionalArg<u64>,
	) -> SCResult<()> {
		let current_time = self.blockchain().get_block_timestamp();
		let start_time = opt_start_time.into_option().unwrap_or(current_time);

		require!(
			nft_nonce > 0,
			"Only Semi-Fungible and Non-Fungible tokens can be auctioned"
		);
		require!(
			min_bid > 0 && min_bid <= max_bid,
			"Min bid can't be 0 or higher than max bid"
		);
		require!(
			accepted_payment_token.is_egld() || accepted_payment_token.is_valid_esdt_identifier(),
			"Invalid accepted payment token"
		);
		require!(deadline > current_time, "Deadline can't be in the past");
		require!(
			start_time >= current_time && start_time < deadline,
			"Invalid start time"
		);

		let marketplace_cut_percentage = self.bid_cut_percentage().get();
		let creator_royalties_percentage = self.get_nft_info(&nft_type, nft_nonce).royalties;

		require!(
			&marketplace_cut_percentage + &creator_royalties_percentage < PERCENTAGE_TOTAL,
			"Marketplace cut plus royalties exceeds 100%"
		);

		let accepted_payment_nft_nonce = if accepted_payment_token.is_egld() {
			0
		} else {
			opt_accepted_payment_token_nonce
				.into_option()
				.unwrap_or_default()
		};

		let auction_id = self.last_valid_auction_id().get() + 1;
		self.last_valid_auction_id().set(&auction_id);

		let sft_max_one_per_user = opt_sft_max_one_per_user.into_option().unwrap_or_default();
		let auction_type = if nft_amount > Self::BigUint::from(NFT_AMOUNT) {
			match sft_max_one_per_user {
				true => AuctionType::SftOnePerUser,
				false => AuctionType::SftAll,
			}
		} else {
			AuctionType::Nft
		};

		self.auction_by_id(auction_id).set(&Auction {
			auctioned_token: EsdtToken {
				token_type: nft_type,
				nonce: nft_nonce,
			},
			nr_auctioned_tokens: nft_amount,
			auction_type,
			auction_status: AuctionStatus::Running,

			payment_token: EsdtToken {
				token_type: accepted_payment_token,
				nonce: accepted_payment_nft_nonce,
			},
			min_bid,
			max_bid,
			start_time,
			deadline,

			original_owner: self.blockchain().get_caller(),
			current_bid: Self::BigUint::zero(),
			current_winner: Address::zero(),
			marketplace_cut_percentage,
			creator_royalties_percentage,
		});

		Ok(())
	}

	#[payable("*")]
	#[endpoint]
	fn bid(
		&self,
		#[payment_token] payment_token: TokenIdentifier,
		#[payment_nonce] payment_token_nonce: u64,
		#[payment_amount] payment_amount: Self::BigUint,
		auction_id: u64,
		nft_type: TokenIdentifier,
		nft_nonce: u64,
	) -> SCResult<()> {
		let mut auction = self.try_get_auction(auction_id)?;
		let caller = self.blockchain().get_caller();
		let current_time = self.blockchain().get_block_timestamp();

		require!(
			auction.auctioned_token.token_type == nft_type
				&& auction.auctioned_token.nonce == nft_nonce,
			"Auction ID does not match the token"
		);
		require!(
			auction.original_owner != caller,
			"Can't bid on your own token"
		);
		require!(
			current_time >= auction.start_time,
			"Auction hasn't started yet"
		);
		require!(current_time < auction.deadline, "Auction ended already");
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
		self.auction_by_id(auction_id).set(&auction);

		Ok(())
	}

	#[endpoint(endAuction)]
	fn end_auction(&self, auction_id: u64) -> SCResult<()> {
		let mut auction = self.try_get_auction(auction_id)?;
		let current_time = self.blockchain().get_block_timestamp();

		require!(
			current_time > auction.deadline || auction.current_bid == auction.max_bid,
			"Auction deadline has not passed nor is the current bid equal to max bid"
		);
		require!(
			auction.auction_status == AuctionStatus::Running,
			"Auction already ended"
		);

		self.distribute_tokens_after_auction_end(&auction);

		if auction.auction_type == AuctionType::SftOnePerUser {
			auction.auction_status = AuctionStatus::SftWaitingForBuyOrOwnerClaim;
			auction.nr_auctioned_tokens -= &NFT_AMOUNT.into();
			self.auction_by_id(auction_id).set(&auction);
		} else {
			self.auction_by_id(auction_id).clear();
		}

		Ok(())
	}

	#[payable("*")]
	#[endpoint(buySftAfterEndAuction)]
	fn buy_sft_after_end_auction(
		&self,
		#[payment_token] payment_token: TokenIdentifier,
		#[payment_nonce] payment_token_nonce: u64,
		#[payment_amount] payment_amount: Self::BigUint,
		auction_id: u64,
		nft_type: TokenIdentifier,
		nft_nonce: u64,
	) -> SCResult<()> {
		let mut auction = self.try_get_auction(auction_id)?;
		let caller = self.blockchain().get_caller();

		require!(
			auction.auctioned_token.token_type == nft_type
				&& auction.auctioned_token.nonce == nft_nonce,
			"Auction ID does not match the token"
		);
		require!(
			auction.auction_status == AuctionStatus::SftWaitingForBuyOrOwnerClaim,
			"Cannot buy SFT for this auction"
		);
		require!(
			payment_token == auction.payment_token.token_type
				&& payment_token_nonce == auction.payment_token.nonce,
			"Wrong token used as payment"
		);
		require!(
			auction.current_bid == payment_amount,
			"Wrong amount paid, must pay equal to current winning bid"
		);

		auction.current_winner = caller;
		self.distribute_tokens_after_auction_end(&auction);

		auction.nr_auctioned_tokens -= &NFT_AMOUNT.into();
		if auction.nr_auctioned_tokens == 0 {
			self.auction_by_id(auction_id).clear();
		} else {
			self.auction_by_id(auction_id).set(&auction);
		}

		Ok(())
	}

	#[endpoint]
	fn withdraw(&self, auction_id: u64) -> SCResult<()> {
		let auction = self.try_get_auction(auction_id)?;
		let caller = self.blockchain().get_caller();

		require!(
			auction.original_owner == caller,
			"Only the original owner can withdraw"
		);
		require!(
			auction.current_bid == 0
				|| auction.auction_status == AuctionStatus::SftWaitingForBuyOrOwnerClaim,
			"Can't withdraw, NFT already has bids"
		);

		self.auction_by_id(auction_id).clear();

		let nft_type = &auction.auctioned_token.token_type;
		let nft_nonce = auction.auctioned_token.nonce;
		let nft_amount = &auction.nr_auctioned_tokens;
		self.transfer_esdt(&caller, nft_type, nft_nonce, &nft_amount, b"returned token");

		Ok(())
	}

	// private

	fn try_get_auction(&self, auction_id: u64) -> SCResult<Auction<Self::BigUint>> {
		require!(
			self.does_auction_exist(auction_id),
			"Auction does not exist"
		);
		Ok(self.auction_by_id(auction_id).get())
	}

	fn calculate_cut_amount(
		&self,
		total_amount: &Self::BigUint,
		cut_percentage: &Self::BigUint,
	) -> Self::BigUint {
		total_amount * cut_percentage / PERCENTAGE_TOTAL.into()
	}

	fn calculate_winning_bid_split(
		&self,
		auction: &Auction<Self::BigUint>,
	) -> BidSplitAmounts<Self::BigUint> {
		let creator_royalties =
			self.calculate_cut_amount(&auction.current_bid, &auction.creator_royalties_percentage);
		let bid_cut_amount =
			self.calculate_cut_amount(&auction.current_bid, &auction.marketplace_cut_percentage);
		let mut seller_amount_to_send = auction.current_bid.clone();
		seller_amount_to_send -= &creator_royalties;
		seller_amount_to_send -= &bid_cut_amount;

		BidSplitAmounts {
			creator: creator_royalties,
			marketplace: bid_cut_amount,
			seller: seller_amount_to_send,
		}
	}

	fn distribute_tokens_after_auction_end(&self, auction: &Auction<Self::BigUint>) {
		let nft_type = &auction.auctioned_token.token_type;
		let nft_nonce = auction.auctioned_token.nonce;

		if auction.current_winner != Address::zero() {
			let nft_info = self.get_nft_info(nft_type, nft_nonce);
			let token_id = &auction.payment_token.token_type;
			let nonce = auction.payment_token.nonce;
			let bid_split_amounts = self.calculate_winning_bid_split(&auction);

			// send part as cut for contract owner
			let owner = self.blockchain().get_owner_address();
			self.transfer_esdt(
				&owner,
				token_id,
				nonce,
				&bid_split_amounts.marketplace,
				b"bid cut for sold token",
			);

			// send part as royalties to creator
			self.transfer_esdt(
				&nft_info.creator,
				token_id,
				nonce,
				&bid_split_amounts.creator,
				b"royalties for sold token",
			);

			// send rest of the bid to original owner
			self.transfer_esdt(
				&auction.original_owner,
				token_id,
				nonce,
				&bid_split_amounts.seller,
				b"sold token",
			);

			// send NFT to auction winner
			let nft_amount_to_send = match auction.auction_type {
				AuctionType::Nft | AuctionType::SftOnePerUser => NFT_AMOUNT.into(),
				_ => auction.nr_auctioned_tokens.clone(),
			};
			self.transfer_esdt(
				&auction.current_winner,
				&nft_type,
				nft_nonce,
				&nft_amount_to_send,
				b"bought token at auction",
			);
		} else {
			// return to original owner
			self.transfer_esdt(
				&auction.original_owner,
				&nft_type,
				nft_nonce,
				&auction.nr_auctioned_tokens,
				b"returned token",
			);
		}
	}

	fn transfer_esdt(
		&self,
		to: &Address,
		token_id: &TokenIdentifier,
		nonce: u64,
		amount: &Self::BigUint,
		data: &'static [u8],
	) {
		// nonce 0 means fungible ESDT or EGLD
		if nonce == 0 {
			self.send()
				.direct(to, &token_id, amount, self.data_or_empty_if_sc(to, data));
		} else {
			self.send().direct_nft(
				to,
				&token_id,
				nonce,
				amount,
				self.data_or_empty_if_sc(to, data),
			);
		}
	}

	fn data_or_empty_if_sc(&self, dest: &Address, data: &'static [u8]) -> &[u8] {
		if self.blockchain().is_smart_contract(dest) {
			&[]
		} else {
			data
		}
	}

	fn get_nft_info(
		&self,
		nft_type: &TokenIdentifier,
		nft_nonce: u64,
	) -> EsdtTokenData<Self::BigUint> {
		self.blockchain().get_esdt_token_data(
			&self.blockchain().get_sc_address(),
			&nft_type,
			nft_nonce,
		)
	}

	fn try_set_bid_cut_percentage(&self, new_cut_percentage: u64) -> SCResult<()> {
		require!(
			new_cut_percentage > 0 && new_cut_percentage < PERCENTAGE_TOTAL,
			"Invalid percentage value, should be between 0 and 10,000"
		);

		self.bid_cut_percentage()
			.set(&Self::BigUint::from(new_cut_percentage));

		Ok(())
	}

	// storage
}
