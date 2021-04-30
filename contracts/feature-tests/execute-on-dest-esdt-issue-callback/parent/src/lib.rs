#![no_std]
#![allow(unused_attributes)]
#![allow(non_snake_case)]

elrond_wasm::imports!();

// Base cost for standalone + estimate cost of actual sc call
const ISSUE_EXPECTED_GAS_COST: u64 = 90_000_000 + 25_000_000;

#[elrond_wasm_derive::callable(ChildProxy)]
pub trait Child {
	fn issueWrappedEgld(
		&self,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: BigUint,
	) -> ContractCall<BigUint, ()>; // payable EGLD
}

#[elrond_wasm_derive::contract(ParentImpl)]
pub trait Parent {
	#[init]
	fn init(&self) {}

	#[payable("EGLD")]
	#[endpoint]
	fn deposit(&self) {}

	#[endpoint(deployChildContract)]
	fn deploy_child_contract(&self, code: BoxedBytes) {
		let child_contract_address = self.send().deploy_contract(
			self.blockchain().get_gas_left(),
			&BigUint::zero(),
			&code,
			CodeMetadata::DEFAULT,
			&ArgBuffer::new(),
		);

		self.child_contract_address().set(&child_contract_address);
	}

	#[payable("EGLD")]
	#[endpoint(executeOnDestIssueToken)]
	fn execute_on_dest_issue_token(
		&self,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: BigUint,
		#[payment] issue_cost: BigUint,
	) {
		let child_contract_adress = self.child_contract_address().get();
		contract_call!(self, child_contract_adress, ChildProxy)
			.with_token_transfer(TokenIdentifier::egld(), issue_cost)
			.issueWrappedEgld(token_display_name, token_ticker, initial_supply)
			.execute_on_dest_context(ISSUE_EXPECTED_GAS_COST, self.send());
	}

	// storage

	#[view(getChildContractAddress)]
	#[storage_mapper("childContractAddress")]
	fn child_contract_address(&self) -> SingleValueMapper<Self::Storage, Address>;
}
