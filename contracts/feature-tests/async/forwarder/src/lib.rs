#![no_std]

mod call_async;
mod call_sync;
mod esdt;
mod nft;
mod roles;
mod sft;
mod storage;
mod vault_proxy;

use call_async::*;
use call_sync::*;
use esdt::*;
use nft::*;
use roles::*;
use sft::*;
use storage::*;

elrond_wasm::imports!();

/// Test contract for investigating contract calls.
/// TODO: split into modules.
#[elrond_wasm_derive::contract(ForwarderImpl)]
pub trait Forwarder {
	#[module(ForwarderAsyncCallModuleImpl)]
	fn async_call_module(&self) -> ForwarderAsyncCallModuleImpl<T, BigInt, BigUint>;

	#[module(ForwarderSyncCallModuleImpl)]
	fn sync_call_module(&self) -> ForwarderSyncCallModuleImpl<T, BigInt, BigUint>;

	#[module(ForwarderEsdtModuleImpl)]
	fn esdt_module(&self) -> ForwarderEsdtModuleImpl<T, BigInt, BigUint>;

	#[module(ForwarderNftModuleImpl)]
	fn nft_module(&self) -> ForwarderNftModuleImpl<T, BigInt, BigUint>;

	#[module(ForwarderSftModuleImpl)]
	fn sft_module(&self) -> ForwarderSftModuleImpl<T, BigInt, BigUint>;

	#[module(ForwarderRolesModuleImpl)]
	fn roles_module(&self) -> ForwarderRolesModuleImpl<T, BigInt, BigUint>;

	#[module(ForwarderStorageModuleImpl)]
	fn storage_module(&self) -> ForwarderStorageModuleImpl<T, BigInt, BigUint>;

	#[init]
	fn init(&self) {}

	#[endpoint]
	fn send_egld(
		&self,
		to: &Address,
		amount: &BigUint,
		#[var_args] opt_data: OptionalArg<BoxedBytes>,
	) {
		let data = match &opt_data {
			OptionalArg::Some(data) => data.as_slice(),
			OptionalArg::None => &[],
		};
		self.send().direct_egld(to, amount, data);
	}

	#[callback]
	fn retrieve_funds_callback(
		&self,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: BigUint,
	) {
		// manual callback forwarding to modules is currently necessary
		self.async_call_module()
			.retrieve_funds_callback(token, payment)
	}

	#[callback]
	fn send_funds_twice_callback(
		&self,
		to: &Address,
		token_identifier: &TokenIdentifier,
		amount: &BigUint,
	) -> AsyncCall<BigUint> {
		// manual callback forwarding to modules is currently necessary
		self.async_call_module()
			.send_funds_twice_callback(to, token_identifier, amount)
	}

	#[callback]
	fn esdt_issue_callback(
		&self,
		caller: &Address,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] returned_tokens: BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		// manual callback forwarding to modules is currently necessary
		self.esdt_module()
			.esdt_issue_callback(caller, token_identifier, returned_tokens, result)
	}

	#[callback]
	fn nft_issue_callback(
		&self,
		caller: &Address,
		#[call_result] result: AsyncCallResult<TokenIdentifier>,
	) {
		// manual callback forwarding to modules is currently necessary
		self.nft_module().nft_issue_callback(caller, result)
	}

	#[callback]
	fn sft_issue_callback(
		&self,
		caller: &Address,
		#[call_result] result: AsyncCallResult<TokenIdentifier>,
	) {
		// manual callback forwarding to modules is currently necessary
		self.sft_module().sft_issue_callback(caller, result)
	}

	#[callback]
	fn change_roles_callback(&self, #[call_result] result: AsyncCallResult<()>) {
		// manual callback forwarding to modules is currently necessary
		self.roles_module().change_roles_callback(result)
	}
}
