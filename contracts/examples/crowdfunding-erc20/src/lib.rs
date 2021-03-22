#![no_std]
#![allow(unused_attributes)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, PartialEq, TypeAbi, Clone, Copy)]
pub enum Status {
	FundingPeriod,
	Successful,
	Failed,
}

#[elrond_wasm_derive::callable(Erc20Proxy)]
pub trait Erc20 {
	fn transferFrom(
		&self,
		sender: &Address,
		recipient: &Address,
		amount: &BigUint,
	) -> ContractCall<BigUint, ()>;

	fn transfer(&self, to: &Address, amount: &BigUint) -> ContractCall<BigUint, ()>;
}

#[elrond_wasm_derive::contract(CrowdfundingImpl)]
pub trait Crowdfunding {
	#[init]
	fn init(&self, target: BigUint, deadline: u64, erc20_contract_address: Address) {
		let my_address: Address = self.get_caller();

		self.set_owner(&my_address);
		self.set_erc20_contract_address(&erc20_contract_address);
		self.set_target(&target);
		self.set_deadline(deadline);
	}

	#[endpoint]
	fn fund(&self, token_amount: BigUint) -> SCResult<AsyncCall<BigUint>> {
		if self.get_block_nonce() > self.get_deadline() {
			return sc_error!("cannot fund after deadline");
		}

		let caller = self.get_caller();
		let erc20_address = self.get_erc20_contract_address();
		let cf_contract_address = self.get_sc_address();

		Ok(contract_call!(self, erc20_address, Erc20Proxy)
			.transferFrom(&caller, &cf_contract_address, &token_amount)
			.async_call()
			.with_callback(
				self.callbacks()
					.transfer_from_callback(&caller, &token_amount),
			))
	}

	#[view]
	fn status(&self) -> Status {
		if self.get_block_nonce() <= self.get_deadline() {
			Status::FundingPeriod
		} else if self.get_sc_balance() >= self.get_target() {
			Status::Successful
		} else {
			Status::Failed
		}
	}

	#[endpoint]
	fn claim(&self) -> SCResult<OptionalResult<AsyncCall<BigUint>>> {
		match self.status() {
			Status::FundingPeriod => sc_error!("cannot claim before deadline"),
			Status::Successful => {
				let caller = self.get_caller();
				if caller != self.get_owner() {
					return sc_error!("only owner can claim successful funding");
				}

				let balance = self.get_total_balance();
				self.set_total_balance(&BigUint::zero());

				let erc20_address = self.get_erc20_contract_address();
				Ok(OptionalResult::Some(
					contract_call!(self, erc20_address, Erc20Proxy)
						.transfer(&caller, &balance)
						.async_call(),
				))
			},
			Status::Failed => {
				let caller = self.get_caller();
				let deposit = self.get_deposit(&caller);

				if deposit > 0 {
					self.set_deposit(&caller, &BigUint::zero());

					let erc20_address = self.get_erc20_contract_address();
					Ok(OptionalResult::Some(
						contract_call!(self, erc20_address, Erc20Proxy)
							.transfer(&caller, &deposit)
							.async_call(),
					))
				} else {
					Ok(OptionalResult::None)
				}
			},
		}
	}

	#[callback]
	fn transfer_from_callback(
		&self,
		#[call_result] result: AsyncCallResult<()>,
		cb_sender: &Address,
		cb_amount: &BigUint,
	) -> OptionalResult<AsyncCall<BigUint>> {
		match result {
			AsyncCallResult::Ok(()) => {
				// transaction started before deadline, ended after -> refund
				if self.get_block_nonce() > self.get_deadline() {
					let erc20_address = self.get_erc20_contract_address();
					return OptionalResult::Some(
						contract_call!(self, erc20_address, Erc20Proxy)
							.transfer(&cb_sender, cb_amount)
							.async_call(),
					);
				}

				let mut deposit = self.get_deposit(&cb_sender);
				deposit += cb_amount;
				self.set_deposit(cb_sender, &deposit);

				let mut balance = self.get_total_balance();
				balance += cb_amount;
				self.set_total_balance(&balance);

				OptionalResult::None
			},
			AsyncCallResult::Err(_) => OptionalResult::None,
		}
	}

	// storage

	#[storage_set("owner")]
	fn set_owner(&self, address: &Address);

	#[view]
	#[storage_get("owner")]
	fn get_owner(&self) -> Address;

	#[storage_set("target")]
	fn set_target(&self, target: &BigUint);

	#[view]
	#[storage_get("target")]
	fn get_target(&self) -> BigUint;

	#[storage_set("deadline")]
	fn set_deadline(&self, deadline: u64);

	#[view]
	#[storage_get("deadline")]
	fn get_deadline(&self) -> u64;

	#[storage_set("deposit")]
	fn set_deposit(&self, donor: &Address, amount: &BigUint);

	#[view]
	#[storage_get("deposit")]
	fn get_deposit(&self, donor: &Address) -> BigUint;

	#[storage_set("erc20_contract_address")]
	fn set_erc20_contract_address(&self, address: &Address);

	#[view]
	#[storage_get("erc20_contract_address")]
	fn get_erc20_contract_address(&self) -> Address;

	#[view]
	#[storage_get("erc20_balance")]
	fn get_total_balance(&self) -> BigUint;

	#[storage_set("erc20_balance")]
	fn set_total_balance(&self, balance: &BigUint);
}
