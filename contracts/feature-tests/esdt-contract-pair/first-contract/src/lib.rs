#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();

use elrond_wasm::HexCallDataSerializer;

const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";
const SECOND_CONTRACT_ACCEPT_ESDT_PAYMENT: &[u8] = b"acceptEsdtPayment";
const SECOND_CONTRACT_REJECT_ESDT_PAYMENT: &[u8] = b"rejectEsdtPayment";

#[elrond_wasm_derive::contract(FirstContractImpl)]
pub trait FirstContract {
	#[init]
	fn init(&self, esdt_token_name: TokenIdentifier, second_contract_address: Address) {
		self.set_contract_esdt_token_name(&esdt_token_name);
		self.set_second_contract_address(&second_contract_address);
	}

	#[payable("*")]
	#[endpoint(transferToSecondContractFull)]
	fn transfer_to_second_contract_full(
		&self,
		#[payment] esdt_value: BigUint,
		#[payment_token] actual_token_name: TokenIdentifier,
	) -> SCResult<()> {
		let expected_token_name = self.get_contract_esdt_token_name();

		require!(esdt_value > 0, "no esdt transfered!");
		require!(actual_token_name == expected_token_name, "Wrong esdt token");

		self.call_esdt_second_contract(
			&expected_token_name,
			&esdt_value,
			&self.get_second_contract_address(),
			SECOND_CONTRACT_ACCEPT_ESDT_PAYMENT,
			&[],
		);

		Ok(())
	}

	#[payable("*")]
	#[endpoint(transferToSecondContractHalf)]
	fn transfer_to_second_contract_half(
		&self,
		#[payment] esdt_value: BigUint,
		#[payment_token] actual_token_name: TokenIdentifier,
	) -> SCResult<()> {
		let expected_token_name = self.get_contract_esdt_token_name();

		require!(esdt_value > 0, "no esdt transfered!");
		require!(actual_token_name == expected_token_name, "Wrong esdt token");

		self.call_esdt_second_contract(
			&expected_token_name,
			&(esdt_value / BigUint::from(2u32)),
			&self.get_second_contract_address(),
			SECOND_CONTRACT_ACCEPT_ESDT_PAYMENT,
			&[],
		);

		Ok(())
	}

	#[payable("*")]
	#[endpoint]
	fn transfer_to_second_contract_rejected(
		&self,
		#[payment] esdt_value: BigUint,
		#[payment_token] actual_token_name: TokenIdentifier,
	) -> SCResult<()> {
		let expected_token_name = self.get_contract_esdt_token_name();

		require!(esdt_value > 0, "no esdt transfered!");
		require!(actual_token_name == expected_token_name, "Wrong esdt token");

		self.call_esdt_second_contract(
			&expected_token_name,
			&(esdt_value / BigUint::from(2u32)),
			&self.get_second_contract_address(),
			SECOND_CONTRACT_REJECT_ESDT_PAYMENT,
			&[],
		);

		Ok(())
	}

	fn call_esdt_second_contract(
		&self,
		esdt_token_name: &TokenIdentifier,
		amount: &BigUint,
		to: &Address,
		func_name: &[u8],
		args: &[BoxedBytes],
	) {
		let mut serializer = HexCallDataSerializer::new(ESDT_TRANSFER_STRING);
		serializer.push_argument_bytes(esdt_token_name.as_esdt_identifier());
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
		serializer.push_argument_bytes(func_name);
		for arg in args {
			serializer.push_argument_bytes(arg.as_slice());
		}

		self.send()
			.async_call_raw(&to, &BigUint::zero(), serializer.as_slice());
	}

	// storage

	#[storage_set("esdtTokenName")]
	fn set_contract_esdt_token_name(&self, esdt_token_name: &TokenIdentifier);

	#[view(getEsdtTokenName)]
	#[storage_get("esdtTokenName")]
	fn get_contract_esdt_token_name(&self) -> TokenIdentifier;

	#[storage_set("secondContractAddress")]
	fn set_second_contract_address(&self, address: &Address);

	#[view(getSecondContractAddress)]
	#[storage_get("secondContractAddress")]
	fn get_second_contract_address(&self) -> Address;
}
