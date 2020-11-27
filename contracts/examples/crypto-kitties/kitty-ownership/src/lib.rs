#![no_std]

imports!();

use elrond_wasm::mut_storage;
use kitty::Kitty;

#[elrond_wasm_derive::contract(KittyOwnershipImp)]
pub trait KittyOwnership {
	#[init]
	fn init(&self) {

	}

	// endpoints

	// private

	fn _transfer(&self, from: &Address, to: &Address, kitty_id: u32) {
		let mut nr_owned_from = self.get_mut_nr_owned_kitties(from);
		*nr_owned_from += 1;

		if from != &Address::zero() {
			let mut nr_owned_to = self.get_mut_nr_owned_kitties(to);
			*nr_owned_to -= 1;

			self.clear_sire_allowed_address();
			self.clear_approved_address();
		}
		
		self.set_kitty_owner(kitty_id, to);
	}

	// checks should be done in the caller function
	// returns the newly created kitten id
	fn _create_new_kitty(&self, matron_id: u32, sire_id: u32, generation: u16, 
		genes: u64, owner: &Address) -> u32 {

		let mut total_kitties = self.get_mut_total_kitties();
		let new_kitty_id = *total_kitties;
		let kitty = Kitty {
			genes,
			birth_time: self.get_block_timestamp(),
			cooldown_end: 0,
			matron_id,
			sire_id,
			siring_with_id: 0,
			nr_children: 0,
			generation
		};

		self.set_kitty_at_id(new_kitty_id, &kitty);
		self._transfer(&Address::zero(), owner, new_kitty_id);

		*total_kitties += 1;

		return new_kitty_id;
	}

	// storage

	#[storage_get_mut("totalKitties")]
	fn get_mut_total_kitties(&self) -> mut_storage!(u32);

	#[storage_get("kitty")]
	fn get_kitty_by_id(&self, id: u32) -> Kitty;

	#[storage_set("kitty")]
	fn set_kitty_at_id(&self, id: u32, kitty: &Kitty);

	#[view(getKittyOwner)]
	#[storage_get("owner")]
	fn get_kitty_owner(&self, id: u32) -> Address;

	#[storage_set("owner")]
	fn set_kitty_owner(&self, id: u32, owner: &Address);

	#[storage_get_mut("nrOwnedKitties")]
	fn get_mut_nr_owned_kitties(&self, address: &Address) -> mut_storage!(u32);

	#[view(getApprovedAddressForKitty)]
	#[storage_get("approvedAddress")]
	fn get_approved_address(&self, id: u32) -> Address;

	#[storage_set("approvedAddress")]
	fn set_approved_address(&self, id: u32, address: &Address);

	#[storage_clear("approvedAddress")]
	fn clear_approved_address(&self);

	#[view(getSireAllowedAddressForKitty)]
	#[storage_get("sireAllowedAddress")]
	fn get_sire_allowed_address(&self, id: u32) -> Address;

	#[storage_set("sireAllowedAddress")]
	fn set_sire_allowed_address(&self, id: u32, address: &Address);

	#[storage_clear("sireAllowedAddress")]
	fn clear_sire_allowed_address(&self);
}
