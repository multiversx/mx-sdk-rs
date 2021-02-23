#![no_std]
#![allow(unused_attributes)]

use elrond_wasm::HexCallDataSerializer;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const TRANSFER_TOKEN_ENDPOINT_NAME: &[u8] = b"safeTransferFrom";

const PERCENTAGE_TOTAL: u8 = 100;

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Auction<BigUint: BigUintApi> {
	pub token_identifier: TokenIdentifier,
	pub min_bid: BigUint,
	pub max_bid: BigUint,
	pub deadline: u64,
	pub original_owner: Address,
	pub current_bid: BigUint,
	pub current_winner: Address,
}

#[derive(TopDecode, TypeAbi)]
pub struct AuctionArgument<BigUint: BigUintApi> {
	pub token_identifier: TokenIdentifier,
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
		nft_id: BigUint,
		args: AuctionArgument<BigUint>,
	) -> SCResult<()> {
		require!(
			self.get_caller() == self.get_token_ownership_contract_address(),
			"Only the token ownership contract may call this function"
		);

		sc_try!(self.try_create_auction(
			&type_id,
			&nft_id,
			&from,
			&args.token_identifier,
			&args.min_bid,
			&args.max_bid,
			args.deadline
		));

		Ok(())
	}

	/// Same `AuctionArgument` is used for all tokens  
	/// `_operator` argument is ignored, but it has to be kept because of the erc1155 standard
	#[endpoint(onERC1155BatchReceived)]
	fn on_erc1155_batch_received(
		&self,
		_operator: Address,
		from: Address,
		type_ids: Vec<BigUint>,
		nft_ids: Vec<BigUint>,
		args: AuctionArgument<BigUint>,
	) -> SCResult<()> {
		require!(
			self.get_caller() == self.get_token_ownership_contract_address(),
			"Only the token ownership contract may call this function"
		);
		require!(
			type_ids.len() == nft_ids.len(),
			"type_ids and nft_ids lengths do not match"
		);

		// Don't have to worry about checking if there are duplicates in the entries,
		// an error here will revert all storage changes automatically
		for (type_id, nft_id) in type_ids.iter().zip(nft_ids.iter()) {
			sc_try!(self.try_create_auction(
				type_id,
				nft_id,
				&from,
				&args.token_identifier,
				&args.min_bid,
				&args.max_bid,
				args.deadline
			));
		}

		Ok(())
	}

	// endpoints - owner-only

	#[endpoint]
	fn claim(&self) -> SCResult<()> {
		only_owner!(self, "Only owner may call this function!");

		let caller = self.get_caller();
		let claimable_funds_mapper = self.get_claimable_funds_mapper();
		for (esdt_token, amount) in claimable_funds_mapper.iter() {
			self.send().direct(&caller, &esdt_token, &amount, b"claim");

			self.clear_claimable_funds(&esdt_token);
		}

		Ok(())
	}

	#[endpoint(setCutPercentage)]
	fn set_percentage_cut_endpoint(&self, new_cut_percentage: u8) -> SCResult<()> {
		only_owner!(self, "Only owner may call this function!");
		require!(
			new_cut_percentage > 0 && new_cut_percentage < PERCENTAGE_TOTAL,
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

	#[payable("*")]
	#[endpoint]
	fn bid(
		&self,
		type_id: BigUint,
		nft_id: BigUint,
		#[payment_token] payment_token: TokenIdentifier,
		#[payment] payment: BigUint,
	) -> SCResult<()> {
		require!(
			self.is_up_for_auction(&type_id, &nft_id),
			"Token is not up for auction"
		);

		let caller = self.get_caller();
		let mut auction = self.get_auction_for_token(&type_id, &nft_id);

		require!(
			auction.original_owner != caller,
			"Can't bid on your own token"
		);
		require!(
			self.get_block_timestamp() < auction.deadline,
			"Auction ended already"
		);
		require!(
			payment_token == auction.token_identifier,
			"Wrong token used as payment"
		);
		require!(
			payment >= auction.min_bid,
			"Bid must be higher than or equal to the min bid"
		);
		require!(
			payment > auction.current_bid,
			"Bid must be higher than the current winning bid"
		);
		require!(
			payment <= auction.max_bid,
			"Bid must be less than or equal to the max bid"
		);

		// refund losing bid
		if auction.current_winner != Address::zero() {
			self.send().direct(
				&auction.current_winner,
				&auction.token_identifier,
				&auction.current_bid,
				b"bit refund",
			);
		}

		// update auction bid and winner
		auction.current_bid = payment;
		auction.current_winner = caller;
		self.set_auction_for_token(&type_id, &nft_id, &auction);

		Ok(())
	}

	#[endpoint(endAuction)]
	fn end_auction(&self, type_id: BigUint, nft_id: BigUint) -> SCResult<()> {
		require!(
			self.is_up_for_auction(&type_id, &nft_id),
			"Token is not up for auction"
		);

		let auction = self.get_auction_for_token(&type_id, &nft_id);

		require!(
			self.get_block_timestamp() > auction.deadline || auction.current_bid == auction.max_bid,
			"auction has not ended yet!"
		);

		self.clear_auction_for_token(&type_id, &nft_id);

		if auction.current_winner != Address::zero() {
			let percentage_cut = self.get_percentage_cut();
			let cut_amount = self.calculate_cut_amount(&auction.current_bid, percentage_cut);
			let amount_to_send = &auction.current_bid - &cut_amount;

			self.add_claimable_funds(&auction.token_identifier, &cut_amount);

			// send part of the bid to the original owner
			self.send().direct(
				&auction.original_owner,
				&auction.token_identifier,
				&amount_to_send,
				b"sold token",
			);

			// send token to winner
			self.asnyc_transfer_token(&type_id, &nft_id, &auction.current_winner);
		} else {
			// return to original owner
			self.asnyc_transfer_token(&type_id, &nft_id, &auction.original_owner);
		}

		Ok(())
	}

	// views

	#[view(isUpForAuction)]
	fn is_up_for_auction(&self, type_id: &BigUint, nft_id: &BigUint) -> bool {
		!self.is_empty_auction_for_token(type_id, nft_id)
	}

	#[view(getAuctionStatus)]
	fn get_auction_status(
		&self,
		type_id: BigUint,
		nft_id: BigUint,
	) -> SCResult<Auction<BigUint>> {
		require!(
			self.is_up_for_auction(&type_id, &nft_id),
			"Token is not up for auction"
		);

		Ok(self.get_auction_for_token(&type_id, &nft_id))
	}

	#[view(getCurrentWinningBid)]
	fn get_current_winning_bid(&self, type_id: BigUint, nft_id: BigUint) -> SCResult<BigUint> {
		require!(
			self.is_up_for_auction(&type_id, &nft_id),
			"Token is not up for auction"
		);

		Ok(self.get_auction_for_token(&type_id, &nft_id).current_bid)
	}

	#[view(getCurrentWinner)]
	fn get_current_winner(&self, type_id: BigUint, nft_id: BigUint) -> SCResult<Address> {
		require!(
			self.is_up_for_auction(&type_id, &nft_id),
			"Token is not up for auction"
		);

		Ok(self
			.get_auction_for_token(&type_id, &nft_id)
			.current_winner)
	}

	// private

	fn try_create_auction(
		&self,
		type_id: &BigUint,
		nft_id: &BigUint,
		original_owner: &Address,
		token: &TokenIdentifier,
		min_bid: &BigUint,
		max_bid: &BigUint,
		deadline: u64,
	) -> SCResult<()> {
		require!(
			!self.is_up_for_auction(&type_id, &nft_id),
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
			&nft_id,
			&Auction {
				token_identifier: token.clone(),
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

	// TODO: Replace with Proxy in the next framework version
	fn asnyc_transfer_token(&self, type_id: &BigUint, nft_id: &BigUint, to: &Address) {
		let sc_own_address = self.get_sc_address();
		let token_ownership_contract_address = self.get_token_ownership_contract_address();
		let mut serializer = HexCallDataSerializer::new(TRANSFER_TOKEN_ENDPOINT_NAME);

		serializer.push_argument_bytes(sc_own_address.as_bytes()); // from
		serializer.push_argument_bytes(to.as_bytes()); // to
		serializer.push_argument_bytes(type_id.to_bytes_be().as_slice()); // type_id
		serializer.push_argument_bytes(nft_id.to_bytes_be().as_slice()); // value
		serializer.push_argument_bytes(&[]); // data

		self.send().async_call_raw(
			&token_ownership_contract_address,
			&BigUint::zero(),
			serializer.as_slice(),
		);
	}

	fn calculate_cut_amount(&self, total_amount: &BigUint, cut_percentage: u8) -> BigUint {
		&(total_amount * &(cut_percentage as u32).into()) / &(PERCENTAGE_TOTAL as u32).into()
	}

	fn add_claimable_funds(&self, esdt_token: &TokenIdentifier, amount: &BigUint) {
		let mut mapper = self.get_claimable_funds_mapper();
		let mut total = mapper.get(esdt_token).unwrap_or_else(|| BigUint::zero());
		total += amount;
		mapper.insert(esdt_token.clone(), total);
	}

	fn clear_claimable_funds(&self, esdt_token: &TokenIdentifier) {
		let mut mapper = self.get_claimable_funds_mapper();
		mapper.insert(esdt_token.clone(), BigUint::zero());
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

	// claimable funds - only after an auction ended and the fixed percentage has been reserved by the SC

	#[storage_mapper("claimableFunds")]
	fn get_claimable_funds_mapper(&self) -> MapMapper<Self::Storage, TokenIdentifier, BigUint>;

	// auction properties for each token

	#[storage_get("auctionForToken")]
	fn get_auction_for_token(&self, type_id: &BigUint, nft_id: &BigUint) -> Auction<BigUint>;

	#[storage_set("auctionForToken")]
	fn set_auction_for_token(
		&self,
		type_id: &BigUint,
		nft_id: &BigUint,
		auction: &Auction<BigUint>,
	);

	#[storage_is_empty("auctionForToken")]
	fn is_empty_auction_for_token(&self, type_id: &BigUint, nft_id: &BigUint) -> bool;

	#[storage_clear("auctionForToken")]
	fn clear_auction_for_token(&self, type_id: &BigUint, nft_id: &BigUint);
}
