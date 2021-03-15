#![no_std]
#![allow(unused_attributes)]

use elrond_wasm::{imports, require, sc_error};
imports!();

mod deposit_info;
use deposit_info::DepositInfo;


#[elrond_wasm_derive::contract(DepositImpl)]
pub trait Deposit {
	
	#[init]
	fn init(&self) {

		let my_address: Address = self.get_caller();
		self.set_owner(&my_address);
	}

	fn get_expiration_round(&self,valability: u64) -> u64{

		let valability_rounds = valability / 6;
		
		return self.get_block_round() + valability_rounds;
	}

	#[payable]
	#[endpoint]
	fn fund(&self, #[payment] payment: BigUint, address: Address, valability: u64) -> SCResult<()> {

		require!(
				payment > 0,
				"amount must be greater than 0"
		);

		let deposit  = &DepositInfo{
			amount : payment,
			depositor_address : self.get_caller(),
			expiration : self.get_expiration_round(valability)
		};

		self.set_deposit(&address,deposit);

		Ok(())
	}


	#[endpoint]
	fn withdraw(&self, address: Address) -> SCResult<()> {

		let deposit = self.get_deposit(&address);

		require!(deposit.expiration < self.get_block_round(),
				"withdrawal has not been available yet")
		;

		self.send_tx(&deposit.depositor_address, &deposit.amount, b"successful withdrawal");
		self.clear_deposit_info(&address);
			
		Ok(())
	}

	#[endpoint]
	fn claim(&self, address: Address, signature: &[u8]) -> SCResult<()> {

		let deposit = self.get_deposit(&address);
		let caller_address: Address = self.get_caller();

		require!(deposit.expiration >= self.get_block_round(),
				"deposit expired"
		);
		require!(self.verify_ed25519(address.as_bytes(), caller_address.as_bytes(), signature),
				"invalid signature"
		);
		
		self.send_tx(&self.get_caller(), &deposit.amount, b"successful claim");
		self.clear_deposit_info(&address);	
		
		
		Ok(())
	}

	#[view(amount)]
	fn get_amount(&self,address: Address) -> SCResult<BigUint>{

		let data = self.get_deposit(&address);

		Ok(data.amount)
	}

	//storage
	#[storage_set("owner")]
	fn set_owner(&self, address: &Address);

	#[view]
	#[storage_get("owner")]
	fn get_owner(&self) -> Address;

	#[storage_set("deposit")]
	fn set_deposit(&self, donor: &Address, deposit_info: &DepositInfo<BigUint>);

	#[view]
	#[storage_get("deposit")]
	fn get_deposit(&self, donor: &Address) -> DepositInfo<BigUint>;

	#[storage_clear("deposit")]
	fn clear_deposit_info(&self, donor: &Address);
}
