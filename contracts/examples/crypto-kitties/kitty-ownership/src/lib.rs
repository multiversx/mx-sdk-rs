#![no_std]

imports!();

use kitty::{Kitty, KittyGenesType};

#[elrond_wasm_derive::contract(KittyOwnershipImpl)]
pub trait KittyOwnership {
	#[init]
	fn init(&self, _gen_zero_kitties: u32) {
		self._create_genesis_kitty();

		// TBD: randomly create gen_zero_kitties
	}

	// endpoints - ERC721 required

	#[endpoint(totalSupply)]
	fn total_supply(&self) -> u32 {
		self.get_total_kitties() - 1 // not counting genesis Kitty
	}

	#[endpoint(balanceOf)]
	fn balance_of(&self, address: &Address) -> u32 {
		self.get_nr_owned_kitties(address)
	}

	#[endpoint(ownerOf)]
	fn owner_of(&self, kitty_id: u32) -> Address {
		self.get_kitty_owner(kitty_id)
	}

	fn approve(&self, to: &Address, kitty_id: u32) -> SCResult<()> {
		let caller = self.get_caller();

		require!(self.get_kitty_owner(kitty_id) == caller,
			"You are not the owner of that kitty!");

		self.set_approved_address(kitty_id, to);
		self.approve_event(&caller, to, kitty_id);

		Ok(())
	}

	fn transfer(&self, to: &Address, kitty_id: u32) -> SCResult<()> {
		let caller = self.get_caller();

		require!(to != &Address::zero(), "Can't transfer to default address 0x0!");
		require!(to != &self.get_sc_address(), "Can't transfer to this contract!");

		require!(self.get_kitty_owner(kitty_id) == caller, 
			"You are not the owner of that kitty!");

		self._transfer(&caller, to, kitty_id);

		Ok(())
	}

	fn transfer_from(&self, from: &Address, to: &Address, kitty_id: u32) -> SCResult<()> {
		let caller = self.get_caller();

		require!(to != &Address::zero(), "Can't transfer to default address 0x0!");
		require!(to != &self.get_sc_address(), "Can't transfer to this contract!");

		require!(&self.get_kitty_owner(kitty_id) == from,
			"Address _from_ is not the owner!");

		require!(self.get_kitty_owner(kitty_id) == caller ||
			self.get_approved_address(kitty_id) == caller,
			"You are not the owner of that kitty nor the approved address!");

		self._transfer(from, to, kitty_id);

		Ok(())
	}

	#[endpoint(tokensOfOwner)]
	fn tokens_of_owner(&self, address: &Address) -> Vec<u32> {
		let nr_owned_kitties = self.get_nr_owned_kitties(address);
		let total_kitties = self.get_total_kitties();
		let mut kitty_list = Vec::new();

		for kitty_id in 1..total_kitties {
			if nr_owned_kitties as usize == kitty_list.len() {
				break;
			}

			if &self.get_kitty_owner(kitty_id) == address {
				kitty_list.push(kitty_id);
			}
		}
		
		kitty_list
	}

	// endpoints - other

	// private

	fn _transfer(&self, from: &Address, to: &Address, kitty_id: u32) {
		let mut nr_owned_from = self.get_nr_owned_kitties(from);
		nr_owned_from -= 1;

		if from != &Address::zero() {
			let mut nr_owned_to = self.get_nr_owned_kitties(to);
			nr_owned_to += 1;

			self.set_nr_owned_kitties(to, nr_owned_to);
			self.clear_sire_allowed_address(kitty_id);
			self.clear_approved_address(kitty_id);
		}
		
		self.set_nr_owned_kitties(from, nr_owned_from);
		self.set_kitty_owner(kitty_id, to);

		self.transfer_event(from, to, kitty_id);
	}

	// checks should be done in the caller function
	// returns the newly created kitten id
	fn _create_new_kitty(&self, matron_id: u32, sire_id: u32, generation: u16, 
		genes: KittyGenesType, owner: &Address) -> u32 {

		let mut total_kitties = self.get_total_kitties();
		let new_kitty_id = total_kitties;
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

		total_kitties += 1;
		self.set_total_kitties(total_kitties);
		self.set_kitty_at_id(new_kitty_id, &kitty);
		
		self._transfer(&Address::zero(), owner, new_kitty_id);

		new_kitty_id
	}

	fn _create_genesis_kitty(&self) {
		let genesis_kitty = Kitty::default();

		self._create_new_kitty(genesis_kitty.matron_id, genesis_kitty.sire_id, 
			genesis_kitty.generation, genesis_kitty.genes, &self.get_sc_address());
	}

	// storage

	#[storage_get("totalKitties")]
	fn get_total_kitties(&self) ->u32;

	#[storage_set("totalKitties")]
	fn set_total_kitties(&self, total_kitties: u32);

	#[storage_get("kitty")]
	fn get_kitty_by_id(&self, kitty_id: u32) -> Kitty;

	#[storage_set("kitty")]
	fn set_kitty_at_id(&self, kitty_id: u32, kitty: &Kitty);

	#[view(getKittyOwner)]
	#[storage_get("owner")]
	fn get_kitty_owner(&self, kitty_id: u32) -> Address;

	#[storage_set("owner")]
	fn set_kitty_owner(&self, kitty_id: u32, owner: &Address);

	#[storage_get("nrOwnedKitties")]
	fn get_nr_owned_kitties(&self, address: &Address) -> u32;

	#[storage_set("nrOwnedKitties")]
	fn set_nr_owned_kitties(&self, address: &Address, nr_owned: u32);

	#[view(getApprovedAddressForKitty)]
	#[storage_get("approvedAddress")]
	fn get_approved_address(&self, kitty_id: u32) -> Address;

	#[storage_set("approvedAddress")]
	fn set_approved_address(&self, kitty_id: u32, address: &Address);

	#[storage_clear("approvedAddress")]
	fn clear_approved_address(&self, kitty_id: u32);

	#[view(getSireAllowedAddressForKitty)]
	#[storage_get("sireAllowedAddress")]
	fn get_sire_allowed_address(&self, kitty_id: u32) -> Address;

	#[storage_set("sireAllowedAddress")]
	fn set_sire_allowed_address(&self, kitty_id: u32, address: &Address);

	#[storage_clear("sireAllowedAddress")]
	fn clear_sire_allowed_address(&self, kitty_id: u32);

	// events

	#[event("0x0000000000000000000000000000000000000000000000000000000000000001")]
	fn transfer_event(&self, from: &Address, to: &Address, token_id: u32);

	#[event("0x0000000000000000000000000000000000000000000000000000000000000002")]
	fn approve_event(&self, owner: &Address, approved: &Address, token_id: u32);
}
