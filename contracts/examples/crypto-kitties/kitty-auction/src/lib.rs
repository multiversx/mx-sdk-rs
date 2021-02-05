#![no_std]

elrond_wasm::imports!();

pub mod auction;
use auction::*;

#[elrond_wasm_derive::callable(KittyOwnershipProxy)]
pub trait KittyOwnership {
	#[rustfmt::skip]
	#[callback(allow_auctioning_callback)]
	fn allowAuctioning(&self, by: Address, kitty_id: u32,
		#[callback_arg] auction_type: AuctionType,
		#[callback_arg] cb_kitty_id: u32,
		#[callback_arg] starting_price: BigUint,
		#[callback_arg] ending_price: BigUint,
		#[callback_arg] deadline: u64,
		#[callback_arg] kitty_owner: Address);

	// not a mistake, same callback for transfer and approveSiringAndReturnKitty

	#[rustfmt::skip]
	#[callback(transfer_callback)]
	fn transfer(&self, to: Address, kitty_id: u32,
		#[callback_arg] cb_kitty_id: u32);

	#[rustfmt::skip]
	#[callback(transfer_callback)]
	fn approveSiringAndReturnKitty(&self,
		approved_address: Address,
		kitty_owner: Address,
		kitty_id: u32,
		#[callback_arg] cb_kitty_id: u32);

	#[rustfmt::skip]
	#[callback(create_gen_zero_kitty_callback)]
	fn createGenZeroKitty(&self);
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
	fn create_and_auction_gen_zero_kitty(&self) -> SCResult<()> {
		only_owner!(self, "Only owner may call this function!");

		let kitty_ownership_contract_address =
			self._get_kitty_ownership_contract_address_or_default();
		if kitty_ownership_contract_address != Address::zero() {
			let proxy = contract_proxy!(self, &kitty_ownership_contract_address, KittyOwnership);
			proxy.createGenZeroKitty();
		} else {
			return sc_error!("Kitty Ownership contract address not set!");
		}

		Ok(())
	}

	#[endpoint]
	fn claim(&self) -> SCResult<()> {
		only_owner!(self, "Only owner may call this function!");

		self.send()
			.direct_egld(&self.get_caller(), &self.get_sc_balance(), b"claim");

		Ok(())
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
	) -> SCResult<()> {
		let deadline = self.get_block_timestamp() + duration;

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
			deadline > self.get_block_timestamp(),
			"deadline can't be in the past!"
		);

		self._create_auction(
			AuctionType::Selling,
			kitty_id,
			starting_price,
			ending_price,
			deadline,
		);

		Ok(())
	}

	#[endpoint(createSiringAuction)]
	fn create_siring_auction(
		&self,
		kitty_id: u32,
		starting_price: BigUint,
		ending_price: BigUint,
		duration: u64,
	) -> SCResult<()> {
		let deadline = self.get_block_timestamp() + duration;

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
			deadline > self.get_block_timestamp(),
			"deadline can't be in the past!"
		);

		self._create_auction(
			AuctionType::Siring,
			kitty_id,
			starting_price,
			ending_price,
			deadline,
		);

		Ok(())
	}

	#[payable("EGLD")]
	#[endpoint]
	fn bid(&self, kitty_id: u32, #[payment] payment: BigUint) -> SCResult<()> {
		require!(
			self.is_up_for_auction(kitty_id),
			"Kitty is not up for auction!"
		);

		let caller = self.get_caller();
		let mut auction = self.get_auction(kitty_id);

		require!(
			caller != auction.kitty_owner,
			"can't bid on your own kitty!"
		);
		require!(
			self.get_block_timestamp() < auction.deadline,
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
	fn end_auction(&self, kitty_id: u32) -> SCResult<()> {
		require!(
			self.is_up_for_auction(kitty_id),
			"kitty is not up for auction!"
		);

		let auction = self.get_auction(kitty_id);

		require!(
			self.get_block_timestamp() > auction.deadline
				|| auction.current_bid == auction.ending_price,
			"auction has not ended yet!"
		);

		if auction.current_winner != Address::zero() {
			match auction.auction_type {
				AuctionType::Selling => {
					self._transfer_to(auction.current_winner, kitty_id);
				},
				AuctionType::Siring => {
					self._approve_siring_and_return_kitty(
						auction.current_winner,
						auction.kitty_owner,
						kitty_id,
					);
				},
			}
		} else {
			// return kitty to its owner
			self._transfer_to(auction.kitty_owner, kitty_id);
		}

		Ok(())
	}

	// private

	fn _create_auction(
		&self,
		auction_type: AuctionType,
		kitty_id: u32,
		starting_price: BigUint,
		ending_price: BigUint,
		deadline: u64,
	) {
		let caller = self.get_caller();

		let kitty_ownership_contract_address =
			self._get_kitty_ownership_contract_address_or_default();
		if kitty_ownership_contract_address != Address::zero() {
			let proxy = contract_proxy!(self, &kitty_ownership_contract_address, KittyOwnership);
			proxy.allowAuctioning(
				caller.clone(),
				kitty_id,
				auction_type,
				kitty_id,
				starting_price,
				ending_price,
				deadline,
				caller,
			);
		}
	}

	fn _start_gen_zero_kitty_auction(&self, kitty_id: u32) {
		let starting_price = self.get_gen_zero_kitty_starting_price();
		let ending_price = self.get_gen_zero_kitty_ending_price();
		let duration = self.get_gen_zero_kitty_auction_duration();
		let deadline = self.get_block_timestamp() + duration;

		let auction = Auction::new(
			AuctionType::Selling,
			&starting_price,
			&ending_price,
			deadline,
			&self.get_sc_address(),
		);

		self.set_auction(kitty_id, &auction);
	}

	fn _transfer_to(&self, address: Address, kitty_id: u32) {
		let kitty_ownership_contract_address =
			self._get_kitty_ownership_contract_address_or_default();
		if kitty_ownership_contract_address != Address::zero() {
			let proxy = contract_proxy!(self, &kitty_ownership_contract_address, KittyOwnership);
			proxy.transfer(address, kitty_id, kitty_id);
		}
	}

	fn _approve_siring_and_return_kitty(
		&self,
		approved_address: Address,
		kitty_owner: Address,
		kitty_id: u32,
	) {
		let kitty_ownership_contract_address =
			self._get_kitty_ownership_contract_address_or_default();
		if kitty_ownership_contract_address != Address::zero() {
			let proxy = contract_proxy!(self, &kitty_ownership_contract_address, KittyOwnership);
			proxy.approveSiringAndReturnKitty(approved_address, kitty_owner, kitty_id, kitty_id);
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
		result: AsyncCallResult<()>,
		#[callback_arg] auction_type: AuctionType,
		#[callback_arg] cb_kitty_id: u32,
		#[callback_arg] starting_price: BigUint,
		#[callback_arg] ending_price: BigUint,
		#[callback_arg] deadline: u64,
		#[callback_arg] kitty_owner: Address,
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
	fn transfer_callback(&self, result: AsyncCallResult<()>, #[callback_arg] cb_kitty_id: u32) {
		match result {
			AsyncCallResult::Ok(()) => {
				let auction = self.get_auction(cb_kitty_id);
				self.clear_auction(cb_kitty_id);

				// send winning bid money to kitty owner
				// condition needed for gen zero kitties, since this sc is their owner
				// and for when no bid was made
				if auction.kitty_owner != self.get_sc_address()
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
		result: AsyncCallResult<()>,
		#[callback_arg] cb_kitty_id: u32,
	) {
		match result {
			AsyncCallResult::Ok(()) => {
				let auction = self.get_auction(cb_kitty_id);

				// transfer kitty back to its owner
				self._transfer_to(auction.kitty_owner, cb_kitty_id);

				// auction data will be cleared in the transfer callback
				// winning bid money will be sent as well
			},
			AsyncCallResult::Err(_) => {
				// this can only fail if the kitty_ownership contract address is invalid
				// nothing to revert in case of error
			},
		}
	}

	#[callback]
	fn create_gen_zero_kitty_callback(&self, result: AsyncCallResult<u32>) {
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
