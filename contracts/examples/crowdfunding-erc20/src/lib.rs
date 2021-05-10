#![no_std]
#![allow(unused_attributes)]
#![allow(unused_variables)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, PartialEq, TypeAbi, Clone, Copy)]
pub enum Status {
	FundingPeriod,
	Successful,
	Failed,
}

#[elrond_wasm_derive::contract]
pub trait Crowdfunding {
	#[init]
	fn init(&self, target: Self::BigUint, deadline: u64, erc20_contract_address: Address) {
		let my_address: Address = self.blockchain().get_caller();

		self.set_owner(&my_address);
		self.set_erc20_contract_address(&erc20_contract_address);
		self.set_target(&target);
		self.set_deadline(deadline);
	}

	#[endpoint]
	fn fund(&self, token_amount: Self::BigUint) -> SCResult<AsyncCall<Self::SendApi>> {
		if self.blockchain().get_block_nonce() > self.get_deadline() {
			return sc_error!("cannot fund after deadline");
		}

		let caller = self.blockchain().get_caller();
		let erc20_address = self.get_erc20_contract_address();
		let cf_contract_address = self.blockchain().get_sc_address();

		Ok(self
			.erc20_proxy(erc20_address)
			.transfer_from(caller.clone(), cf_contract_address, token_amount.clone())
			.async_call()
			.with_callback(
				self.callbacks()
					.transfer_from_callback(caller, token_amount),
			))
	}

	#[view]
	fn status(&self) -> Status {
		if self.blockchain().get_block_nonce() <= self.get_deadline() {
			Status::FundingPeriod
		} else if self.blockchain().get_sc_balance() >= self.get_target() {
			Status::Successful
		} else {
			Status::Failed
		}
	}

	#[endpoint]
	fn claim(&self) -> SCResult<OptionalResult<AsyncCall<Self::SendApi>>> {
		match self.status() {
			Status::FundingPeriod => sc_error!("cannot claim before deadline"),
			Status::Successful => {
				let caller = self.blockchain().get_caller();
				if caller != self.get_owner() {
					return sc_error!("only owner can claim successful funding");
				}

				let balance = self.get_total_balance();
				self.set_total_balance(&Self::BigUint::zero());

				let erc20_address = self.get_erc20_contract_address();
				Ok(OptionalResult::Some(
					self.erc20_proxy(erc20_address)
						.transfer(caller, balance)
						.async_call(),
				))
			},
			Status::Failed => {
				let caller = self.blockchain().get_caller();
				let deposit = self.get_deposit(&caller);

				if deposit > 0 {
					self.set_deposit(&caller, &Self::BigUint::zero());

					let erc20_address = self.get_erc20_contract_address();
					Ok(OptionalResult::Some(
						self.erc20_proxy(erc20_address)
							.transfer(caller, deposit)
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
		cb_sender: Address,
		cb_amount: Self::BigUint,
	) -> OptionalResult<AsyncCall<Self::SendApi>> {
		match result {
			AsyncCallResult::Ok(()) => {
				// transaction started before deadline, ended after -> refund
				if self.blockchain().get_block_nonce() > self.get_deadline() {
					let erc20_address = self.get_erc20_contract_address();
					return OptionalResult::Some(
						self.erc20_proxy(erc20_address)
							.transfer(cb_sender, cb_amount)
							.async_call(),
					);
				}

				let mut deposit = self.get_deposit(&cb_sender);
				deposit += &cb_amount;
				self.set_deposit(&cb_sender, &deposit);

				let mut balance = self.get_total_balance();
				balance += &cb_amount;
				self.set_total_balance(&balance);

				OptionalResult::None
			},
			AsyncCallResult::Err(_) => OptionalResult::None,
		}
	}

	// proxy

	#[proxy]
	fn erc20_proxy(&self, to: Address) -> erc20::ProxyObj<Self::SendApi>;

	// storage

	#[storage_set("owner")]
	fn set_owner(&self, address: &Address);

	#[view]
	#[storage_get("owner")]
	fn get_owner(&self) -> Address;

	#[storage_set("target")]
	fn set_target(&self, target: &Self::BigUint);

	#[view]
	#[storage_get("target")]
	fn get_target(&self) -> Self::BigUint;

	#[storage_set("deadline")]
	fn set_deadline(&self, deadline: u64);

	#[view]
	#[storage_get("deadline")]
	fn get_deadline(&self) -> u64;

	#[storage_set("deposit")]
	fn set_deposit(&self, donor: &Address, amount: &Self::BigUint);

	#[view]
	#[storage_get("deposit")]
	fn get_deposit(&self, donor: &Address) -> Self::BigUint;

	#[storage_set("erc20_contract_address")]
	fn set_erc20_contract_address(&self, address: &Address);

	#[view]
	#[storage_get("erc20_contract_address")]
	fn get_erc20_contract_address(&self) -> Address;

	#[view]
	#[storage_get("erc20_balance")]
	fn get_total_balance(&self) -> Self::BigUint;

	#[storage_set("erc20_balance")]
	fn set_total_balance(&self, balance: &Self::BigUint);
}
