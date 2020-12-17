#![no_std]

imports!();

pub mod auction;
use auction::*;

#[elrond_wasm_derive::callable(KittyOwnershipProxy)]
pub trait KittyOwnership {
	#[rustfmt::skip]
	#[callback(allow_auctioning_callback)]
	fn allowAuctioning(&self, 
		kitty_id: u32, 
		by: Address,
		#[callback_arg] auction_type: AuctionType,
		#[callback_arg] cb_kitty_id: u32,
		#[callback_arg] starting_price: BigUint,
		#[callback_arg] ending_price: BigUint,
		#[callback_arg] deadline: u64,
		#[callback_arg] kitty_owner: Address);
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
		require!(starting_price < ending_price, 
			"starting price must be less than ending price!");
		require!(deadline < self.get_block_timestamp(), 
			"deadline can't be in the past!");

		let caller = self.get_caller();

		let kitty_ownership_contract_address = self.get_kitty_ownership_contract_address();
		let proxy = contract_proxy!(self, &kitty_ownership_contract_address, KittyOwnership);
		proxy.allowAuctioning(kitty_id, caller.clone(), AuctionType::Selling,
			kitty_id, starting_price, ending_price, deadline, caller);

		Ok(())
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
