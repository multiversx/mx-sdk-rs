#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const ACCEPTED_TRANSFER_ANSWER: u32 = 0xbc197c81;

#[derive(TopEncode, TopDecode)]
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
	#[init]
	fn init(&self, token_ownership_contract_address: Address) {
		self.set_token_ownership_contract_address(&token_ownership_contract_address);
	}

	// endpoints

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
		require!(
			self.is_empty_auction_for_token(&type_id, &token_id),
			"There is already an auction for that token"
		);
		require!(
			args.min_bid > 0 && args.min_bid <= args.max_bid,
			"Min bid can't be 0 or higher than max bid"
		);
		require!(
			args.deadline > self.get_block_timestamp(),
			"Deadline can't be in the past"
		);

		self.set_auction_for_token(
			&type_id,
			&token_id,
			&Auction {
				min_bid: args.min_bid,
				max_bid: args.max_bid,
				deadline: args.deadline,
				original_owner: from,
				current_bid: BigUint::zero(),
				current_winner: Address::zero(),
			},
		);

		Ok(ACCEPTED_TRANSFER_ANSWER)
	}

	/// same AuctionArgument is used for all tokens
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
		require!(
			args.min_bid > 0 && args.min_bid <= args.max_bid,
			"Min bid can't be 0 or higher than max bid"
		);
		require!(
			args.deadline > self.get_block_timestamp(),
			"Deadline can't be in the past"
		);

		// Don't have to worry about checking if there are duplicates in the entries,
		// an error here will revert all storage changes automatically
		for (type_id, token_id) in type_ids.iter().zip(token_ids.iter()) {
			require!(
				self.is_empty_auction_for_token(&type_id, &token_id),
				"There is already an auction for that token"
			);

			self.set_auction_for_token(
				&type_id,
				&token_id,
				&Auction {
					min_bid: args.min_bid.clone(),
					max_bid: args.max_bid.clone(),
					deadline: args.deadline.clone(),
					original_owner: from.clone(),
					current_bid: BigUint::zero(),
					current_winner: Address::zero(),
				},
			);
		}

		Ok(ACCEPTED_TRANSFER_ANSWER)
	}

	// private

	// storage

	// token ownership contract, i.e. the erc1155 SC

	#[storage_get("tokenOwnershipContractAddress")]
	fn get_token_ownership_contract_address(&self) -> Address;

	#[storage_set("tokenOwnershipContractAddress")]
	fn set_token_ownership_contract_address(&self, token_ownership_contract_address: &Address);

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
}
