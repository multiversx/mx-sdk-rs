#![no_std]

imports!();

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

	#[rustfmt::skip]
	#[callback(transfer_callback)]
	fn transfer(&self, to: Address, kitty_id: u32, 
		#[callback_arg] cb_kitty_id: u32);

	#[rustfmt::skip]
	#[callback(approve_siring_callback)]
	fn approveSiring(&self, address: Address, kitty_id: u32,
		#[callback_arg] cb_kitty_id: u32);
}

#[elrond_wasm_derive::contract(KittyAuctionImpl)]
pub trait KittyAuction {
	#[init]
	fn init(&self, kitty_ownership_contract_address: Address) {
		self.set_kitty_ownership_contract_address(&kitty_ownership_contract_address);
	}

	// endpoints

	#[endpoint(isUpForAuction)]
	fn is_up_for_auction(&self, kitty_id: u32) -> bool {
		!self.is_empty_auction(kitty_id)
	}

	#[endpoint(createSaleAuction)]
	fn create_sale_auction(&self, kitty_id: u32, 
		starting_price: BigUint, ending_price: BigUint, deadline: u64) -> SCResult<()> {

		require!(!self.is_up_for_auction(kitty_id), "kitty already auctioned!");
		require!(starting_price > 0, "starting price must be higher than 0!");
		require!(starting_price < ending_price, 
			"starting price must be less than ending price!");
		require!(deadline < self.get_block_timestamp(), 
			"deadline can't be in the past!");

		self._create_auction(AuctionType::Selling, kitty_id, 
			starting_price, ending_price, deadline);

		Ok(())
	}

	#[endpoint(createSiringAuction)]
	fn create_siring_auction(&self, kitty_id: u32, 
		starting_price: BigUint, ending_price: BigUint, deadline: u64) -> SCResult<()> {

		require!(!self.is_up_for_auction(kitty_id), "kitty already auctioned!");
		require!(starting_price > 0, "starting price must be higher than 0!");
		require!(starting_price < ending_price, 
			"starting price must be less than ending price!");
		require!(deadline < self.get_block_timestamp(), 
			"deadline can't be in the past!");

		self._create_auction(AuctionType::Siring, kitty_id, 
			starting_price, ending_price, deadline);

		Ok(())
	}

	#[payable]
	#[endpoint]
	fn bid(&self, kitty_id: u32, #[payment] payment: BigUint) -> SCResult<()> {
		require!(self.is_up_for_auction(kitty_id), "Kitty is not up for auction!");

		let caller = self.get_caller();
		let mut auction = self.get_auction(kitty_id);

		require!(caller != auction.kitty_owner, "can't bid on your own kitty!");
		require!(self.get_block_timestamp() < auction.deadline,
			"auction ended already!");
		require!(payment >= auction.starting_price, 
			"bid amount must be higher than or equal to starting price!");
		require!(payment > auction.current_bid, 
			"bid amount must be higher than current winning bid!");
		require!(payment <= auction.ending_price,
			"bid amount must be less than or equal to ending price!");

		// refund losing bid
		if auction.current_winner != Address::zero() {
			self.send_tx(&auction.current_winner, &auction.current_bid, "bid refund");
		}

		// update auction bid and winner
		auction.current_bid = payment;
		auction.current_winner = caller;
		self.set_auction(kitty_id, &auction);

		Ok(())
	}

	#[endpoint(endAuction)]
	fn end_auction(&self, kitty_id: u32) -> SCResult<()> {
		require!(self.is_up_for_auction(kitty_id), "kitty is not up for auction!");

		let auction = self.get_auction(kitty_id);

		require!(self.get_block_timestamp() >= auction.deadline || 
			auction.current_bid == auction.ending_price, "auction has not ended yet!");

		match auction.auction_type {
			AuctionType::Selling => {
				self._transfer_to(auction.current_winner, kitty_id);
			},
			AuctionType::Siring => {
				self._approve_siring(auction.current_winner, kitty_id);
			}
		}

		Ok(())
	}

	// callbacks

	#[callback]
	fn allow_auctioning_callback(&self, 
		result: AsyncCallResult<()>,
		#[callback_arg] auction_type: AuctionType,
		#[callback_arg] cb_kitty_id: u32,
		#[callback_arg] starting_price: BigUint,
		#[callback_arg] ending_price: BigUint,
		#[callback_arg] deadline: u64,
		#[callback_arg] kitty_owner: Address
	) {
		match result {
			AsyncCallResult::Ok(()) => {
				let auction = Auction::new(auction_type, 
					&starting_price, &ending_price, 
					deadline, &kitty_owner);

				self.set_auction(cb_kitty_id, &auction);
			}
			AsyncCallResult::Err(_) => {
				// nothing to revert in case of error
			}
		}
	}

	#[callback]
	fn transfer_callback(&self,
		result: AsyncCallResult<()>,
		#[callback_arg] cb_kitty_id: u32
	) {
		match result {
			AsyncCallResult::Ok(()) => {
				self.clear_auction(cb_kitty_id);
			}
			AsyncCallResult::Err(_) => {
				// this can only fail if the kitty_genes contract address is invalid
				// nothing to revert in case of error
			}
		}
	}

	#[callback]
	fn approve_siring_callback(&self,
		result: AsyncCallResult<()>,
		#[callback_arg] cb_kitty_id: u32
	) {
		match result {
			AsyncCallResult::Ok(()) => {
				let auction = self.get_auction(cb_kitty_id);

				// transfer kitty back to its owner
				self._transfer_to(auction.kitty_owner, cb_kitty_id);

				// auction data will be cleared in the transfer callback
			}
			AsyncCallResult::Err(_) => {
				// this can only fail if the kitty_genes contract address is invalid
				// nothing to revert in case of error
			}
		}
	}

	// private

	fn _create_auction(&self, auction_type: AuctionType, kitty_id: u32, 
		starting_price: BigUint, ending_price: BigUint, deadline: u64) {

		let caller = self.get_caller();

		let kitty_ownership_contract_address = self.get_kitty_ownership_contract_address();
		let proxy = contract_proxy!(self, &kitty_ownership_contract_address, KittyOwnership);
		proxy.allowAuctioning(caller.clone(), kitty_id, auction_type,
			kitty_id, starting_price, ending_price, deadline, caller);
	}

	fn _transfer_to(&self, address: Address, kitty_id: u32) {
		let kitty_ownership_contract_address = self.get_kitty_ownership_contract_address();
		let proxy = contract_proxy!(self, &kitty_ownership_contract_address, KittyOwnership);
		proxy.transfer(address, kitty_id, kitty_id);
	}

	fn _approve_siring(&self, address: Address, kitty_id: u32) {
		let kitty_ownership_contract_address = self.get_kitty_ownership_contract_address();
		let proxy = contract_proxy!(self, &kitty_ownership_contract_address, KittyOwnership);
		proxy.approveSiring(address, kitty_id, kitty_id);
	}

	// storage

	// general

	#[storage_get("kittyOwnershipContractAddress")]
	fn get_kitty_ownership_contract_address(&self) -> Address;

	#[storage_set("kittyOwnershipContractAddress")]
	fn set_kitty_ownership_contract_address(&self, address: &Address);

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
