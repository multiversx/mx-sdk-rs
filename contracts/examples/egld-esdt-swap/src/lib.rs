#![no_std]

use elrond_wasm::HexCallDataSerializer;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u
const ESDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] = [
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xff,
];

const ESDT_ISSUE_COST: u64 = 5000000000000000000; // 5 eGLD

const ESDT_ISSUE_STRING: &[u8] = b"issue";
const ESDT_MINT_STRING: &[u8] = b"mint";

const EGLD_DECIMALS: u8 = 18;

#[derive(TopEncode, TopDecode)]
pub enum EsdtOperation<BigUint: BigUintApi> {
	None,
	Issue,
	Mint(BigUint), // amount minted
}

#[elrond_wasm_derive::contract(EgldEsdtSwapImpl)]
pub trait EgldEsdtSwap {
	#[init]
	fn init(&self) {}

	// endpoints - owner-only

	#[payable("EGLD")]
	#[endpoint(performWrappedEgldIssue)]
	fn perform_wrapped_egld_issue(
		&self,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: BigUint,
		#[payment] payment: BigUint,
	) -> SCResult<()> {
		only_owner!(self, "only owner may call this function");

		require!(
			self.is_empty_wrapped_egld_token_identifier(),
			"wrapped egld was already issued"
		);
		require!(
			payment == BigUint::from(ESDT_ISSUE_COST),
			"Wrong payment, should pay exactly 5 eGLD for ESDT token issue"
		);

		self.issue_esdt_token(
			&token_display_name,
			&token_ticker,
			&initial_supply,
			EGLD_DECIMALS,
		);

		Ok(())
	}

	#[endpoint(mintWrappedEgld)]
	fn mint_wrapped_egld(&self, amount: BigUint) -> SCResult<()> {
		only_owner!(self, "only owner may call this function");

		require!(
			!self.is_empty_wrapped_egld_token_identifier(),
			"Wrapped eGLD was not issued yet"
		);

		self.mint_esdt_token(&self.get_wrapped_egld_token_identifier(), &amount);

		Ok(())
	}

	// endpoints

	#[payable("EGLD")]
	#[endpoint(wrapEgld)]
	fn wrap_egld(&self, #[payment] payment: BigUint) -> SCResult<()> {
		require!(payment > 0, "Payment must be more than 0");
		require!(
			!self.is_empty_wrapped_egld_token_identifier(),
			"Wrapped eGLD was not issued yet"
		);

		let wrapped_egld_left = self.get_wrapped_egld_remaining();
		require!(
			wrapped_egld_left > payment,
			"Contract does not have enough wrapped eGLD. Please try again once more is minted."
		);

		self.substract_total_wrapped_egld(&payment);

		self.send().direct_esdt_via_transf_exec(
			&self.get_caller(),
			self.get_wrapped_egld_token_identifier().as_slice(),
			&payment,
			b"wrapping",
		);

		Ok(())
	}

	#[payable("*")]
	#[endpoint(unwrapEgld)]
	fn unwrap_egld(
		&self,
		#[payment] wrapped_egld_payment: BigUint,
		#[payment_token] token_identifier: TokenIdentifier,
	) -> SCResult<()> {
		require!(
			!self.is_empty_wrapped_egld_token_identifier(),
			"Wrapped eGLD was not issued yet"
		);
		require!(token_identifier.is_esdt(), "Only ESDT tokens accepted");

		let wrapped_egld_token_identifier = self.get_wrapped_egld_token_identifier();

		require!(
			token_identifier == wrapped_egld_token_identifier,
			"Wrong esdt token"
		);

		require!(wrapped_egld_payment > 0, "Must pay more than 0 tokens!");
		// this should never happen, but we'll check anyway
		require!(
			wrapped_egld_payment <= self.get_sc_balance(),
			"Contract does not have enough funds"
		);

		self.add_total_wrapped_egld(&wrapped_egld_payment);

		// 1 wrapped eGLD = 1 eGLD, so we pay back the same amount
		self.send()
			.direct_egld(&self.get_caller(), &wrapped_egld_payment, b"unwrapping");

		Ok(())
	}

	#[view(getLockedEgldBalance)]
	fn get_locked_egld_balance() -> BigUint {
		self.get_sc_balance()
	}

	// private

	fn add_total_wrapped_egld(&self, amount: &BigUint) {
		let mut total_wrapped = self.get_wrapped_egld_remaining();
		total_wrapped += amount;
		self.set_wrapped_egld_remaining(&total_wrapped);
	}

	fn substract_total_wrapped_egld(&self, amount: &BigUint) {
		let mut total_wrapped = self.get_wrapped_egld_remaining();
		total_wrapped -= amount;
		self.set_wrapped_egld_remaining(&total_wrapped);
	}

	fn issue_esdt_token(
		&self,
		token_display_name: &BoxedBytes,
		token_ticker: &BoxedBytes,
		initial_supply: &BigUint,
		num_decimals: u8,
	) {
		let mut serializer = HexCallDataSerializer::new(ESDT_ISSUE_STRING);

		serializer.push_argument_bytes(token_display_name.as_slice());
		serializer.push_argument_bytes(token_ticker.as_slice());
		serializer.push_argument_bytes(&initial_supply.to_bytes_be());
		serializer.push_argument_bytes(&[num_decimals]);

		serializer.push_argument_bytes(&b"canFreeze"[..]);
		serializer.push_argument_bytes(&b"false"[..]);

		serializer.push_argument_bytes(&b"canWipe"[..]);
		serializer.push_argument_bytes(&b"false"[..]);

		serializer.push_argument_bytes(&b"canPause"[..]);
		serializer.push_argument_bytes(&b"false"[..]);

		serializer.push_argument_bytes(&b"canMint"[..]);
		serializer.push_argument_bytes(&b"true"[..]);

		serializer.push_argument_bytes(&b"canBurn"[..]);
		serializer.push_argument_bytes(&b"true"[..]);

		serializer.push_argument_bytes(&b"canChangeOwner"[..]);
		serializer.push_argument_bytes(&b"false"[..]);

		serializer.push_argument_bytes(&b"canUpgrade"[..]);
		serializer.push_argument_bytes(&b"true"[..]);

		// save data for callback
		self.set_temporary_storage_esdt_operation(&self.get_tx_hash(), &EsdtOperation::Issue);

		self.send().async_call_raw(
			&Address::from(ESDT_SYSTEM_SC_ADDRESS_ARRAY),
			&BigUint::from(ESDT_ISSUE_COST),
			serializer.as_slice(),
		);
	}

	fn mint_esdt_token(&self, token_identifier: &TokenIdentifier, amount: &BigUint) {
		let mut serializer = HexCallDataSerializer::new(ESDT_MINT_STRING);
		serializer.push_argument_bytes(token_identifier.as_slice());
		serializer.push_argument_bytes(&amount.to_bytes_be());

		// save data for callback
		self.set_temporary_storage_esdt_operation(
			&self.get_tx_hash(),
			&EsdtOperation::Mint(amount.clone()),
		);

		self.send().async_call_raw(
			&Address::from(ESDT_SYSTEM_SC_ADDRESS_ARRAY),
			&BigUint::zero(),
			serializer.as_slice(),
		);
	}

	// callbacks

	#[callback_raw]

	fn callback_raw(&self, #[var_args] result: AsyncCallResult<VarArgs<BoxedBytes>>) {
		let success = match result {
			AsyncCallResult::Ok(_) => true,
			AsyncCallResult::Err(_) => false,
		};
		let original_tx_hash = self.get_tx_hash();

		let esdt_operation = self.get_temporary_storage_esdt_operation(&original_tx_hash);
		match esdt_operation {
			EsdtOperation::None => return,
			EsdtOperation::Issue => self.perform_esdt_issue_callback(success),
			EsdtOperation::Mint(amount) => self.perform_esdt_mint_callback(success, &amount),
		};

		self.clear_temporary_storage_esdt_operation(&original_tx_hash);
	}

	fn perform_esdt_issue_callback(&self, success: bool) {
		// callback is called with ESDTTransfer of the newly issued token, with the amount requested,
		// so we can get the token identifier and amount from the call data
		if success {
			let token_identifier = self.call_value().token();
			let initial_supply = self.call_value().esdt_value();

			self.set_wrapped_egld_remaining(&initial_supply);
			self.set_wrapped_egld_token_identifier(&token_identifier);
		}
		// nothing to do in case of error
	}

	fn perform_esdt_mint_callback(&self, success: bool, amount: &BigUint) {
		if success {
			self.add_total_wrapped_egld(amount);
		}
		// nothing to do in case of error
	}

	// storage

	// 1 eGLD = 1 wrapped eGLD, and they are interchangeable through this contract

	#[view(getWrappedEgldTokenIdentifier)]
	#[storage_get("wrappedEgldTokenIdentifier")]
	fn get_wrapped_egld_token_identifier(&self) -> TokenIdentifier;

	#[storage_set("wrappedEgldTokenIdentifier")]
	fn set_wrapped_egld_token_identifier(&self, token_identifier: &TokenIdentifier);

	#[storage_is_empty("wrappedEgldTokenIdentifier")]
	fn is_empty_wrapped_egld_token_identifier(&self) -> bool;

	#[view(getWrappedEgldRemaining)]
	#[storage_get("wrappedEgldRemaining")]
	fn get_wrapped_egld_remaining(&self) -> BigUint;

	#[storage_set("wrappedEgldRemaining")]
	fn set_wrapped_egld_remaining(&self, wrapped_egld_remaining: &BigUint);

	// temporary storage for ESDT operations. Used in callback.

	#[storage_get("temporaryStorageEsdtOperation")]
	fn get_temporary_storage_esdt_operation(
		&self,
		original_tx_hash: &H256,
	) -> EsdtOperation<BigUint>;

	#[storage_set("temporaryStorageEsdtOperation")]
	fn set_temporary_storage_esdt_operation(
		&self,
		original_tx_hash: &H256,
		esdt_operation: &EsdtOperation<BigUint>,
	);

	#[storage_clear("temporaryStorageEsdtOperation")]
	fn clear_temporary_storage_esdt_operation(&self, original_tx_hash: &H256);
}
