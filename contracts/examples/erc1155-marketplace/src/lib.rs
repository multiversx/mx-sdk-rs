#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const ACCEPTED_TRANSFER_ANSWER: u32 = 0xbc197c81;

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Auction<BigUint: BigUintApi> {
	pub min_bid: BigUint,
	pub max_bid: BigUint,
	pub deadline: u64,
	pub original_owner: Address,
	pub current_bid: BigUint,
	pub current_winner: Address,
}

#[derive(TopDecode, TypeAbi)]
pub struct AuctionArgument<BigUint: BigUintApi> {
	pub min_bid: BigUint,
	pub max_bid: BigUint,
	pub deadline: u64,
}

#[elrond_wasm_derive::contract(Erc1155MarketplaceImpl)]
pub trait Erc1155Marketplace {
	/// `bid_cut_percentage` is the cut that the contract takes from any sucessful bid
	#[init]
	fn init(&self, token_ownership_contract_address: Address, bid_cut_percentage: u8) {
		self.set_token_ownership_contract_address(&token_ownership_contract_address);
		self.set_percentage_cut(bid_cut_percentage);
	}

	// endpoints - Token ownership contract only

	/// `_operator` argument is ignored, but it has to be kept because of the erc1155 standard
	#[endpoint(onERC1155Received)]
	fn on_erc1155_received(
		&self,
		_operator: Address,
		from: Address,
		type_id: BigUint,
		token_id: BigUint,
		args: AuctionArgument<BigUint>,
	) -> SCResult<u32> {
		require!(
			self.get_caller() == self.get_token_ownership_contract_address(),
			"Only the token ownership contract may call this function"
		);

		sc_try!(self.try_create_auction(
			&type_id,
			&token_id,
			&from,
			&args.min_bid,
			&args.max_bid,
			args.deadline
		));

		Ok(ACCEPTED_TRANSFER_ANSWER)
	}

	/// Same `AuctionArgument` is used for all tokens  
	/// `_operator` argument is ignored, but it has to be kept because of the erc1155 standard
	#[endpoint(onERC1155BatchReceived)]
	fn on_erc1155_batch_received(
		&self,
		_operator: Address,
		from: Address,
		type_ids: Vec<BigUint>,
		token_ids: Vec<BigUint>,
		args: AuctionArgument<BigUint>,
	) -> SCResult<u32> {
		require!(
			self.get_caller() == self.get_token_ownership_contract_address(),
			"Only the token ownership contract may call this function"
		);
		require!(
			type_ids.len() == token_ids.len(),
			"type_ids and token_ids lengths do not match"
		);

		// Don't have to worry about checking if there are duplicates in the entries,
		// an error here will revert all storage changes automatically
		for (type_id, token_id) in type_ids.iter().zip(token_ids.iter()) {
			sc_try!(self.try_create_auction(
				type_id,
				token_id,
				&from,
				&args.min_bid,
				&args.max_bid,
				args.deadline
			));
		}

		Ok(ACCEPTED_TRANSFER_ANSWER)
	}

	// endpoints - owner-only

	#[endpoint]
	fn claim(&self) -> SCResult<()> {
		only_owner!(self, "Only owner may call this function!");

		self.send()
			.direct_egld(&self.get_caller(), &self.get_sc_balance(), b"claim");

		Ok(())
	}

	#[endpoint(setCutPercentage)]
	fn set_percentage_cut_endpoint(&self, new_cut_percentage: u8) -> SCResult<()> {
		only_owner!(self, "Only owner may call this function!");
		require!(
			new_cut_percentage > 0 && new_cut_percentage < 100,
			"Invalid percentage value, should be between 0 and 100"
		);

		self.set_percentage_cut(new_cut_percentage);

		Ok(())
	}

	fn set_token_ownership_contract_address_endpoint(&self, new_address: Address) -> SCResult<()> {
		only_owner!(self, "Only owner may call this function!");
		require!(!new_address.is_zero(), "Cannot set to zero address");
		// TODO: Also check the address with IsSmartContractAddress() once it's added to the API

		self.set_token_ownership_contract_address(&new_address);

		Ok(())
	}

	// endpoints

	

	// views

	#[view(isUpForAuction)]
	fn is_up_for_auction(&self, type_id: &BigUint, token_id: &BigUint) -> bool {
		!self.is_empty_auction_for_token(type_id, token_id)
	}

	#[view(getAuctionStatus)]
	fn get_auction_status(
		&self,
		type_id: BigUint,
		token_id: BigUint,
	) -> SCResult<Auction<BigUint>> {
		require!(
			self.is_up_for_auction(&type_id, &token_id),
			"Token is not up for auction"
		);

		Ok(self.get_auction_for_token(&type_id, &token_id))
	}

	#[view(getCurrentWinningBid)]
	fn get_current_winning_bid(&self, type_id: BigUint, token_id: BigUint) -> SCResult<BigUint> {
		require!(
			self.is_up_for_auction(&type_id, &token_id),
			"Token is not up for auction"
		);

		Ok(self.get_auction_for_token(&type_id, &token_id).current_bid)
	}

	#[view(getCurrentWinner)]
	fn get_current_winner(&self, type_id: BigUint, token_id: BigUint) -> SCResult<Address> {
		require!(
			self.is_up_for_auction(&type_id, &token_id),
			"Token is not up for auction"
		);

		Ok(self
			.get_auction_for_token(&type_id, &token_id)
			.current_winner)
	}

	// private

	fn try_create_auction(
		&self,
		type_id: &BigUint,
		token_id: &BigUint,
		original_owner: &Address,
		min_bid: &BigUint,
		max_bid: &BigUint,
		deadline: u64,
	) -> SCResult<()> {
		require!(
			!self.is_up_for_auction(&type_id, &token_id),
			"There is already an auction for that token"
		);
		require!(
			min_bid > &0 && min_bid <= max_bid,
			"Min bid can't be 0 or higher than max bid"
		);
		require!(
			deadline > self.get_block_timestamp(),
			"Deadline can't be in the past"
		);

		self.set_auction_for_token(
			&type_id,
			&token_id,
			&Auction {
				min_bid: min_bid.clone(),
				max_bid: max_bid.clone(),
				deadline,
				original_owner: original_owner.clone(),
				current_bid: BigUint::zero(),
				current_winner: Address::zero(),
			},
		);

		Ok(())
	}

	// storage

	// token ownership contract, i.e. the erc1155 SC

	#[storage_get("tokenOwnershipContractAddress")]
	fn get_token_ownership_contract_address(&self) -> Address;

	#[storage_set("tokenOwnershipContractAddress")]
	fn set_token_ownership_contract_address(&self, token_ownership_contract_address: &Address);

	// percentage taken from winning bids

	#[view(getPercentageCut)]
	#[storage_get("percentageCut")]
	fn get_percentage_cut(&self) -> u8;

	#[storage_set("percentageCut")]
	fn set_percentage_cut(&self, bid_cut_percentage: u8);

	// auction properties for each token

	#[storage_get("auctionForToken")]
	fn get_auction_for_token(&self, type_id: &BigUint, token_id: &BigUint) -> Auction<BigUint>;

	#[storage_set("auctionForToken")]
	fn set_auction_for_token(
		&self,
		type_id: &BigUint,
		token_id: &BigUint,
		auction: &Auction<BigUint>,
	);

	#[storage_is_empty("auctionForToken")]
	fn is_empty_auction_for_token(&self, type_id: &BigUint, token_id: &BigUint) -> bool;

	#[storage_clear("auctionForToken")]
	fn clear_auction_for_token(&self, type_id: &BigUint, token_id: &BigUint);
}
