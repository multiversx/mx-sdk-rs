use hex_literal::hex;

use crate::{
	api::BigUintApi,
	types::{Address, BoxedBytes, ContractCall, TokenIdentifier},
};

/// Address of the system smart contract that manages ESDT.
/// Bech32: erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u
pub const ESDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] =
	hex!("000000000000000000010000000000000000000000000000000000000002ffff");

pub fn esdt_system_sc_address() -> Address {
	Address::from(ESDT_SYSTEM_SC_ADDRESS_ARRAY)
}

/// Proxy for the ESDT system smart contract.
/// Unlike other contract proxies, this one has a fixed address,
/// so the proxy object doesn't really contain any data, it is more of a placeholder.
pub struct ESDTSystemSmartContractProxy<BigUint: BigUintApi> {
	_phantom: core::marker::PhantomData<BigUint>,
}

impl<BigUint: BigUintApi> ESDTSystemSmartContractProxy<BigUint> {
	pub fn new() -> Self {
		ESDTSystemSmartContractProxy {
			_phantom: core::marker::PhantomData,
		}
	}

	/// Produces a contract call to the ESDT system SC,
	/// which causes it to issue a new ESDT token.
	pub fn issue(
		&self,
		issue_cost: BigUint,
		token_display_name: &BoxedBytes,
		token_ticker: &BoxedBytes,
		initial_supply: &BigUint,
		num_decimals: usize,
		can_freeze: bool,
		can_wipe: bool,
		can_pause: bool,
		can_mint: bool,
		can_burn: bool,
		can_change_owner: bool,
		can_upgrade: bool,
	) -> ContractCall<BigUint> {
		let mut contract_call = ContractCall::new(
			esdt_system_sc_address(),
			TokenIdentifier::egld(),
			issue_cost,
			BoxedBytes::from(&b"issue"[..]),
		);

		contract_call.push_argument_raw_bytes(token_display_name.as_slice());
		contract_call.push_argument_raw_bytes(token_ticker.as_slice());
		contract_call.push_argument_raw_bytes(&initial_supply.to_bytes_be());
		contract_call.push_argument_raw_bytes(&num_decimals.to_be_bytes());

		contract_call.push_argument_raw_bytes(&b"canFreeze"[..]);
		contract_call.push_argument_raw_bytes(bool_name_bytes(can_freeze));

		contract_call.push_argument_raw_bytes(&b"canWipe"[..]);
		contract_call.push_argument_raw_bytes(bool_name_bytes(can_wipe));

		contract_call.push_argument_raw_bytes(&b"canPause"[..]);
		contract_call.push_argument_raw_bytes(bool_name_bytes(can_pause));

		contract_call.push_argument_raw_bytes(&b"canMint"[..]);
		contract_call.push_argument_raw_bytes(bool_name_bytes(can_mint));

		contract_call.push_argument_raw_bytes(&b"canBurn"[..]);
		contract_call.push_argument_raw_bytes(bool_name_bytes(can_burn));

		contract_call.push_argument_raw_bytes(&b"canChangeOwner"[..]);
		contract_call.push_argument_raw_bytes(bool_name_bytes(can_change_owner));

		contract_call.push_argument_raw_bytes(&b"canUpgrade"[..]);
		contract_call.push_argument_raw_bytes(bool_name_bytes(can_upgrade));

		contract_call
	}

	/// Produces a contract call to the ESDT system SC,
	/// which causes it to issue a new ESDT token.
	pub fn mint(&self, token_identifier: &[u8], amount: &BigUint) -> ContractCall<BigUint> {
		let mut contract_call = esdt_system_sc_call_no_args(b"mint");

		contract_call.push_argument_raw_bytes(token_identifier);
		contract_call.push_argument_raw_bytes(&amount.to_bytes_be());

		contract_call
	}

	/// The manager of an ESDT token may choose to suspend all transactions of the token,
	/// except minting, freezing/unfreezing and wiping.
	pub fn pause(&self, token_identifier: &[u8]) -> ContractCall<BigUint> {
		let mut contract_call = esdt_system_sc_call_no_args(b"pause");

		contract_call.push_argument_raw_bytes(token_identifier);

		contract_call
	}

	/// The manager of an ESDT token may choose to suspend all transactions of the token,
	/// except minting, freezing/unfreezing and wiping.
	pub fn unpause(&self, token_identifier: &[u8]) -> ContractCall<BigUint> {
		let mut contract_call = esdt_system_sc_call_no_args(b"unPause");

		contract_call.push_argument_raw_bytes(token_identifier);

		contract_call
	}

	/// The manager of an ESDT token may freeze the tokens held by a specific account.
	/// As a consequence, no tokens may be transferred to or from the frozen account.
	/// Freezing and unfreezing the tokens of an account are operations designed to help token managers to comply with regulations.
	pub fn freeze(&self, token_identifier: &[u8], address: &Address) -> ContractCall<BigUint> {
		let mut contract_call = esdt_system_sc_call_no_args(b"freeze");

		contract_call.push_argument_raw_bytes(token_identifier);
		contract_call.push_argument_raw_bytes(address.as_bytes());

		contract_call
	}

	/// The reverse operation of `freeze`, unfreezing, will allow further transfers to and from the account.
	pub fn unfreeze(&self, token_identifier: &[u8], address: &Address) -> ContractCall<BigUint> {
		let mut contract_call = esdt_system_sc_call_no_args(b"unFreeze");

		contract_call.push_argument_raw_bytes(token_identifier);
		contract_call.push_argument_raw_bytes(address.as_bytes());

		contract_call
	}

	/// The manager of an ESDT token may wipe out all the tokens held by a frozen account.
	/// This operation is similar to burning the tokens, but the account must have been frozen beforehand,
	/// and it must be done by the token manager.
	/// Wiping the tokens of an account is an operation designed to help token managers to comply with regulations.
	pub fn wipe(&self, token_identifier: &[u8], address: &Address) -> ContractCall<BigUint> {
		let mut contract_call = esdt_system_sc_call_no_args(b"wipe");

		contract_call.push_argument_raw_bytes(token_identifier);
		contract_call.push_argument_raw_bytes(address.as_bytes());

		contract_call
	}
}

fn esdt_system_sc_call_no_args<BigUint: BigUintApi>(endpoint_name: &[u8]) -> ContractCall<BigUint> {
	ContractCall::new(
		esdt_system_sc_address(),
		TokenIdentifier::egld(),
		BigUint::zero(),
		endpoint_name.into(),
	)
}

const TRUE_BYTES: &[u8] = b"true";
const FALSE_BYTES: &[u8] = b"false";

fn bool_name_bytes(b: bool) -> &'static [u8] {
	if b {
		TRUE_BYTES
	} else {
		FALSE_BYTES
	}
}
