#![no_std]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();

pub mod auction;
use auction::*;

#[elrond_wasm_derive::callable(KittyOwnershipProxy)]
pub trait KittyOwnership {
	fn allowAuctioning(&self, by: Address, kitty_id: u32) -> ContractCall<BigUint, ()>;

	fn transfer(&self, to: Address, kitty_id: u32) -> ContractCall<BigUint, ()>;

	fn approveSiringAndReturnKitty(
		&self,
		approved_address: Address,
		kitty_owner: Address,
		kitty_id: u32,
	) -> ContractCall<BigUint, ()>;

	fn createGenZeroKitty(&self) -> ContractCall<BigUint, ()>;
}

#[elrond_wasm_derive::contract(KittyAuctionImpl)]
pub trait KittyAuction {
	#[init]
	fn init(
		&self,
		gen_zero_kitty_starting_price: BigUint,
		gen_zero_kitty_ending_price: BigUint,
		gen_zero_kitty_auction_duration: u64,
		#[var_args] opt_kitty_ownership_contract_address: OptionalArg<Address>,
	) {
		self.set_gen_zero_kitty_starting_price(&gen_zero_kitty_starting_price);
		self.set_gen_zero_kitty_ending_price(&gen_zero_kitty_ending_price);
		self.set_gen_zero_kitty_auction_duration(gen_zero_kitty_auction_duration);

		match opt_kitty_ownership_contract_address {
			OptionalArg::Some(addr) => self.set_kitty_ownership_contract_address(&addr),
			OptionalArg::None => {},
		}
	}

	// endpoints - owner-only

	#[endpoint(setKittyOwnershipContractAddress)]
	fn set_kitty_ownership_contract_address_endpoint(&self, address: Address) -> SCResult<()> {
		only_owner!(self, "Only owner may call this function!");

		self.set_kitty_ownership_contract_address(&address);

		Ok(())
	}

	#[endpoint(createAndAuctionGenZeroKitty)]
	fn create_and_auction_gen_zero_kitty(&self) -> SCResult<AsyncCall<BigUint>> {
		only_owner!(self, "Only owner may call this function!");

		let kitty_ownership_contract_address =
			self._get_kitty_ownership_contract_address_or_default();
		if kitty_ownership_contract_address != Address::zero() {
			Ok(
				contract_call!(self, kitty_ownership_contract_address, KittyOwnershipProxy)
					.createGenZeroKitty()
					.async_call()
					.with_callback(self.callbacks().create_gen_zero_kitty_callback()),
			)
		} else {
			sc_error!("Kitty Ownership contract address not set!")
		}
	}

	// views

	#[view(isUpForAuction)]
	fn is_up_for_auction(&self, kitty_id: u32) -> bool {
		!self.is_empty_auction(kitty_id)
	}

	#[view(getAuctionStatus)]
	fn get_auction_status(&self, kitty_id: u32) -> SCResult<Auction<BigUint>> {
		require!(
			self.is_up_for_auction(kitty_id),
			"Kitty is not up for auction!"
		);

		Ok(self.get_auction(kitty_id))
	}

	#[view(getCurrentWinningBid)]
	fn get_current_winning_bid(&self, kitty_id: u32) -> SCResult<BigUint> {
		require!(
			self.is_up_for_auction(kitty_id),
			"Kitty is not up for auction!"
		);

		Ok(self.get_auction(kitty_id).current_bid)
	}

	// endpoints

	#[endpoint(createSaleAuction)]
	fn create_sale_auction(
		&self,
		kitty_id: u32,
		starting_price: BigUint,
		ending_price: BigUint,
		duration: u64,
	) -> SCResult<OptionalResult<AsyncCall<BigUint>>> {
		let deadline = self.blockchain().get_block_timestamp() + duration;

		require!(
			!self.is_up_for_auction(kitty_id),
			"kitty already auctioned!"
		);
		require!(starting_price > 0, "starting price must be higher than 0!");
		require!(
			starting_price < ending_price,
			"starting price must be less than ending price!"
		);
		require!(
			deadline > self.blockchain().get_block_timestamp(),
			"deadline can't be in the past!"
		);

		Ok(self._create_auction(
			AuctionType::Selling,
			kitty_id,
			starting_price,
			ending_price,
			deadline,
		))
	}

	#[endpoint(createSiringAuction)]
	fn create_siring_auction(
		&self,
		kitty_id: u32,
		starting_price: BigUint,
		ending_price: BigUint,
		duration: u64,
	) -> SCResult<OptionalResult<AsyncCall<BigUint>>> {
		let deadline = self.blockchain().get_block_timestamp() + duration;

		require!(
			!self.is_up_for_auction(kitty_id),
			"kitty already auctioned!"
		);
		require!(starting_price > 0, "starting price must be higher than 0!");
		require!(
			starting_price < ending_price,
			"starting price must be less than ending price!"
		);
		require!(
			deadline > self.blockchain().get_block_timestamp(),
			"deadline can't be in the past!"
		);

		Ok(self._create_auction(
			AuctionType::Siring,
			kitty_id,
			starting_price,
			ending_price,
			deadline,
		))
	}

	#[payable("EGLD")]
	#[endpoint]
	fn bid(&self, kitty_id: u32, #[payment] payment: BigUint) -> SCResult<()> {
		require!(
			self.is_up_for_auction(kitty_id),
			"Kitty is not up for auction!"
		);

		let caller = self.blockchain().get_caller();
		let mut auction = self.get_auction(kitty_id);

		require!(
			caller != auction.kitty_owner,
			"can't bid on your own kitty!"
		);
		require!(
			self.blockchain().get_block_timestamp() < auction.deadline,
			"auction ended already!"
		);
		require!(
			payment >= auction.starting_price,
			"bid amount must be higher than or equal to starting price!"
		);
		require!(
			payment > auction.current_bid,
			"bid amount must be higher than current winning bid!"
		);
		require!(
			payment <= auction.ending_price,
			"bid amount must be less than or equal to ending price!"
		);

		// refund losing bid
		if auction.current_winner != Address::zero() {
			self.send()
				.direct_egld(&auction.current_winner, &auction.current_bid, b"bid refund");
		}

		// update auction bid and winner
		auction.current_bid = payment;
		auction.current_winner = caller;
		self.set_auction(kitty_id, &auction);

		Ok(())
	}

	#[endpoint(endAuction)]
	fn end_auction(&self, kitty_id: u32) -> SCResult<OptionalResult<AsyncCall<BigUint>>> {
		require!(
			self.is_up_for_auction(kitty_id),
			"kitty is not up for auction!"
		);

		let auction = self.get_auction(kitty_id);

		require!(
			self.blockchain().get_block_timestamp() > auction.deadline
				|| auction.current_bid == auction.ending_price,
			"auction has not ended yet!"
		);

		if auction.current_winner != Address::zero() {
			match auction.auction_type {
				AuctionType::Selling => Ok(self._transfer_to(auction.current_winner, kitty_id)),
				AuctionType::Siring => Ok(self._approve_siring_and_return_kitty(
					auction.current_winner,
					auction.kitty_owner,
					kitty_id,
				)),
			}
		} else {
			// return kitty to its owner
			Ok(self._transfer_to(auction.kitty_owner, kitty_id))
		}
	}

	// private

	fn _create_auction(
		&self,
		auction_type: AuctionType,
		kitty_id: u32,
		starting_price: BigUint,
		ending_price: BigUint,
		deadline: u64,
	) -> OptionalResult<AsyncCall<BigUint>> {
		let caller = self.blockchain().get_caller();

		let kitty_ownership_contract_address =
			self._get_kitty_ownership_contract_address_or_default();
		if kitty_ownership_contract_address != Address::zero() {
			OptionalResult::Some(
				contract_call!(self, kitty_ownership_contract_address, KittyOwnershipProxy)
					.allowAuctioning(caller.clone(), kitty_id)
					.async_call()
					.with_callback(self.callbacks().allow_auctioning_callback(
						auction_type,
						kitty_id,
						starting_price,
						ending_price,
						deadline,
						caller,
					)),
			)
		} else {
			OptionalResult::None
		}
	}

	fn _start_gen_zero_kitty_auction(&self, kitty_id: u32) {
		let starting_price = self.get_gen_zero_kitty_starting_price();
		let ending_price = self.get_gen_zero_kitty_ending_price();
		let duration = self.get_gen_zero_kitty_auction_duration();
		let deadline = self.blockchain().get_block_timestamp() + duration;

		let auction = Auction::new(
			AuctionType::Selling,
			&starting_price,
			&ending_price,
			deadline,
			&self.blockchain().get_sc_address(),
		);

		self.set_auction(kitty_id, &auction);
	}

	fn _transfer_to(&self, address: Address, kitty_id: u32) -> OptionalResult<AsyncCall<BigUint>> {
		let kitty_ownership_contract_address =
			self._get_kitty_ownership_contract_address_or_default();
		if kitty_ownership_contract_address != Address::zero() {
			OptionalResult::Some(
				contract_call!(self, kitty_ownership_contract_address, KittyOwnershipProxy)
					.transfer(address, kitty_id)
					.async_call()
					.with_callback(self.callbacks().transfer_callback(kitty_id)),
			)
		} else {
			OptionalResult::None
		}
	}

	fn _approve_siring_and_return_kitty(
		&self,
		approved_address: Address,
		kitty_owner: Address,
		kitty_id: u32,
	) -> OptionalResult<AsyncCall<BigUint>> {
		let kitty_ownership_contract_address =
			self._get_kitty_ownership_contract_address_or_default();
		if kitty_ownership_contract_address != Address::zero() {
			OptionalResult::Some(
				contract_call!(self, kitty_ownership_contract_address, KittyOwnershipProxy)
					.approveSiringAndReturnKitty(approved_address, kitty_owner, kitty_id)
					// not a mistake, same callback for transfer and approveSiringAndReturnKitty
					.async_call()
					.with_callback(self.callbacks().transfer_callback(kitty_id)),
			)
		} else {
			OptionalResult::None
		}
	}

	fn _get_kitty_ownership_contract_address_or_default(&self) -> Address {
		if self.is_empty_kitty_ownership_contract_address() {
			Address::zero()
		} else {
			self.get_kitty_ownership_contract_address()
		}
	}

	// callbacks

	#[callback]
	fn allow_auctioning_callback(
		&self,
		#[call_result] result: AsyncCallResult<()>,
		auction_type: AuctionType,
		cb_kitty_id: u32,
		starting_price: BigUint,
		ending_price: BigUint,
		deadline: u64,
		kitty_owner: Address,
	) {
		match result {
			AsyncCallResult::Ok(()) => {
				let auction = Auction::new(
					auction_type,
					&starting_price,
					&ending_price,
					deadline,
					&kitty_owner,
				);

				self.set_auction(cb_kitty_id, &auction);
			},
			AsyncCallResult::Err(_) => {
				// nothing to revert in case of error
			},
		}
	}

	#[callback]
	fn transfer_callback(&self, #[call_result] result: AsyncCallResult<()>, cb_kitty_id: u32) {
		match result {
			AsyncCallResult::Ok(()) => {
				let auction = self.get_auction(cb_kitty_id);
				self.clear_auction(cb_kitty_id);

				// send winning bid money to kitty owner
				// condition needed for gen zero kitties, since this sc is their owner
				// and for when no bid was made
				if auction.kitty_owner != self.blockchain().get_sc_address()
					&& auction.current_winner != Address::zero()
				{
					self.send().direct_egld(
						&auction.kitty_owner,
						&auction.current_bid,
						b"sold kitty",
					);
				}
			},
			AsyncCallResult::Err(_) => {
				// this can only fail if the kitty_ownership contract address is invalid
				// nothing to revert in case of error
			},
		}
	}

	#[callback]
	fn approve_siring_callback(
		&self,
		#[call_result] result: AsyncCallResult<()>,
		cb_kitty_id: u32,
	) -> OptionalResult<AsyncCall<BigUint>> {
		match result {
			AsyncCallResult::Ok(()) => {
				let auction = self.get_auction(cb_kitty_id);

				// transfer kitty back to its owner
				self._transfer_to(auction.kitty_owner, cb_kitty_id)

				// auction data will be cleared in the transfer callback
				// winning bid money will be sent as well
			},
			AsyncCallResult::Err(_) => {
				// this can only fail if the kitty_ownership contract address is invalid
				// nothing to revert in case of error
				OptionalResult::None
			},
		}
	}

	#[callback]
	fn create_gen_zero_kitty_callback(&self, #[call_result] result: AsyncCallResult<u32>) {
		match result {
			AsyncCallResult::Ok(kitty_id) => {
				self._start_gen_zero_kitty_auction(kitty_id);
			},
			AsyncCallResult::Err(_) => {
				// this can only fail if the kitty_ownership contract address is invalid
				// nothing to revert in case of error
			},
		}
	}

	// storage

	// general

	#[storage_get("kittyOwnershipContractAddress")]
	fn get_kitty_ownership_contract_address(&self) -> Address;

	#[storage_set("kittyOwnershipContractAddress")]
	fn set_kitty_ownership_contract_address(&self, address: &Address);

	#[storage_is_empty("kittyOwnershipContractAddress")]
	fn is_empty_kitty_ownership_contract_address(&self) -> bool;

	// gen zero kitty

	#[storage_get("genZeroKittyStartingPrice")]
	fn get_gen_zero_kitty_starting_price(&self) -> BigUint;

	#[storage_set("genZeroKittyStartingPrice")]
	fn set_gen_zero_kitty_starting_price(&self, price: &BigUint);

	#[storage_get("genZeroKittyEndingPrice")]
	fn get_gen_zero_kitty_ending_price(&self) -> BigUint;

	#[storage_set("genZeroKittyEndingPrice")]
	fn set_gen_zero_kitty_ending_price(&self, price: &BigUint);

	#[storage_get("genZeroKittyAuctionDuration")]
	fn get_gen_zero_kitty_auction_duration(&self) -> u64;

	#[storage_set("genZeroKittyAuctionDuration")]
	fn set_gen_zero_kitty_auction_duration(&self, duration: u64);

	// auction

	#[storage_get("auction")]
	fn get_auction(&self, kitty_id: u32) -> Auction<BigUint>;

	#[storage_set("auction")]
	fn set_auction(&self, kitty_id: u32, auction: &Auction<BigUint>);

	#[storage_is_empty("auction")]
	fn is_empty_auction(&self, kitty_id: u32) -> bool;

	#[storage_clear("auction")]
	fn clear_auction(&self, kitty_id: u32);
}
