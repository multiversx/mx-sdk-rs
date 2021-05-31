use elrond_codec::TopEncode;

use super::{BigIntApi, BigUintApi, EllipticCurveApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::hex_call_data::HexCallDataSerializer;
use crate::types::{Address, ArgBuffer, AsyncCall, BoxedBytes, CodeMetadata, TokenIdentifier, Vec};

pub const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";
pub const ESDT_NFT_TRANSFER_STRING: &[u8] = b"ESDTNFTTransfer";

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
#[allow(clippy::too_many_arguments)] // TODO: some arguments should be grouped though
pub trait SendApi: ErrorApi + Clone + Sized {
	/// The type of the payment arguments.
	/// Not named `BigUint` to avoid name collisions in types that implement multiple API traits.
	type AmountType: BigUintApi + 'static;

	/// Not used by `SendApi`, but forwarded to the proxy traits.
	type ProxyBigInt: BigIntApi + 'static;

	type ProxyEllipticCurve: EllipticCurveApi<BigUint = Self::AmountType> + 'static;

	/// Not used by `SendApi`, but forwarded to the proxy traits.
	type ProxyStorage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

	/// Required for ESDTNFTTransfer.
	/// Same as the implementation from BlockchainApi.
	fn get_sc_address(&self) -> Address;

	/// Sends EGLD to a given address, directly.
	/// Used especially for sending EGLD to regular accounts.
	fn direct_egld(&self, to: &Address, amount: &Self::AmountType, data: &[u8]);

	/// Sends EGLD to an address (optionally) and executes like an async call, but without callback.
	fn direct_egld_execute(
		&self,
		to: &Address,
		amount: &Self::AmountType,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]>;

	/// Sends an ESDT token to a given address, directly.
	/// Used especially for sending ESDT to regular accounts.
	///
	/// Unlike sending ESDT via async call, this method can be called multiple times per transaction.
	fn direct_esdt_via_transf_exec(
		&self,
		to: &Address,
		token: &[u8],
		amount: &Self::AmountType,
		data: &[u8],
	) -> Result<(), &'static [u8]> {
		self.direct_esdt_execute(to, token, amount, 0, data, &ArgBuffer::new())
	}

	/// Sends ESDT to an address and executes like an async call, but without callback.
	fn direct_esdt_execute(
		&self,
		to: &Address,
		token: &[u8],
		amount: &Self::AmountType,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]>;

	/// Sends ESDT NFT to an address and executes like an async call, but without callback.
	fn direct_esdt_nft_execute(
		&self,
		to: &Address,
		token: &[u8],
		nonce: u64,
		amount: &Self::AmountType,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]>;

	/// Sends either EGLD or an ESDT token to the target address,
	/// depending on what token identifier was specified.
	fn direct(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		amount: &Self::AmountType,
		data: &[u8],
	) {
		if token.is_egld() {
			self.direct_egld(to, amount, data);
		} else {
			let _ = self.direct_esdt_via_transf_exec(to, token.as_esdt_identifier(), amount, data);
		}
	}

	/// Sends ESDT tokens to the target address. Handles any type of ESDT.
	/// Note: this does not work with EGLD, use only with ESDT.
	fn transfer_tokens(
		&self,
		token: &TokenIdentifier,
		nonce: u64,
		amount: &Self::AmountType,
		to: &Address,
	) {
		if amount > &0 {
			if nonce == 0 {
				let _ =
					self.direct_esdt_via_transf_exec(to, token.as_esdt_identifier(), amount, &[]);
			} else {
				let _ = self.direct_esdt_nft_via_transfer_exec(
					to,
					token.as_esdt_identifier(),
					nonce,
					amount,
					&[],
				);
			}
		}
	}

	/// Performs a simple ESDT transfer, but via async call.
	/// This is the preferred way to send ESDT.
	fn direct_esdt_via_async_call(
		&self,
		to: &Address,
		esdt_token_name: &[u8],
		amount: &Self::AmountType,
		data: &[u8],
	) -> ! {
		let mut serializer = HexCallDataSerializer::new(ESDT_TRANSFER_STRING);
		serializer.push_argument_bytes(esdt_token_name);
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
		if !data.is_empty() {
			serializer.push_argument_bytes(data);
		}
		self.async_call_raw(&to, &Self::AmountType::zero(), serializer.as_slice())
	}

	/// Sends either EGLD or an ESDT token to the target address,
	/// depending on what token identifier was specified.
	/// In case of ESDT it performs an async call.
	fn direct_via_async_call(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		amount: &Self::AmountType,
		data: &[u8],
	) {
		if token.is_egld() {
			let _ = self.direct_egld(to, amount, data);
		} else {
			self.direct_esdt_via_async_call(to, token.as_esdt_identifier(), amount, data);
		}
	}

	/// Sends an asynchronous call to another contract.
	/// Calling this method immediately terminates tx execution.
	/// Using it directly is generally discouraged.
	///
	/// The data is expected to be of the form `functionName@<arg1-hex>@<arg2-hex>@...`.
	/// Use a `HexCallDataSerializer` to prepare this field.
	fn async_call_raw(&self, to: &Address, amount: &Self::AmountType, data: &[u8]) -> !;

	/// Sends an asynchronous call to another contract, with either EGLD or ESDT value.
	/// The `token` argument decides which one it will be.
	/// Calling this method immediately terminates tx execution.
	fn async_call(&self, async_call: AsyncCall<Self>) -> ! {
		self.async_call_raw(
			&async_call.to,
			&async_call.egld_payment,
			async_call.hex_data.as_slice(),
		)
	}

	/// Deploys a new contract in the same shard.
	/// Unlike `async_call_raw`, the deployment is synchronous and tx execution continues afterwards.
	/// Also unlike `async_call_raw`, it uses an argument buffer to pass arguments
	fn deploy_contract(
		&self,
		gas: u64,
		amount: &Self::AmountType,
		code: &BoxedBytes,
		code_metadata: CodeMetadata,
		arg_buffer: &ArgBuffer,
	) -> Address;

	/// Same shard, in-line execution of another contract.
	fn execute_on_dest_context_raw(
		&self,
		gas: u64,
		address: &Address,
		value: &Self::AmountType,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes>;

	/// Same shard, in-line execution of another contract.
	/// Allows the contract to specify which result range to extract as sync call result.
	/// This is a workaround to handle nested sync calls.
	/// Please do not use this method unless there is absolutely no other option.
	/// Will be eliminated after some future Arwen hook redesign.
	/// `range_closure` takes the number of results before, the number of results after,
	/// and is expected to return the start index (inclusive) and end index (exclusive).
	fn execute_on_dest_context_raw_custom_result_range<F>(
		&self,
		gas: u64,
		address: &Address,
		value: &Self::AmountType,
		function: &[u8],
		arg_buffer: &ArgBuffer,
		range_closure: F,
	) -> Vec<BoxedBytes>
	where
		F: FnOnce(usize, usize) -> (usize, usize);

	fn execute_on_dest_context_by_caller_raw(
		&self,
		gas: u64,
		address: &Address,
		value: &Self::AmountType,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes>;

	fn execute_on_same_context_raw(
		&self,
		gas: u64,
		address: &Address,
		value: &Self::AmountType,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	);

	/// Used to store data between async call and callback.
	fn storage_store_tx_hash_key(&self, data: &[u8]);

	/// Used to store data between async call and callback.
	fn storage_load_tx_hash_key(&self) -> BoxedBytes;

	/// Allows synchronously calling a local function by name. Execution is resumed afterwards.
	/// You should never have to call this function directly.
	/// Use the other specific methods instead.
	fn call_local_esdt_built_in_function(&self, gas: u64, function: &[u8], arg_buffer: &ArgBuffer);

	/// Allows synchronous minting of ESDT tokens. Execution is resumed afterwards.
	fn esdt_local_mint(&self, gas: u64, token: &[u8], amount: &Self::AmountType) {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(token);
		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.call_local_esdt_built_in_function(gas, b"ESDTLocalMint", &arg_buffer);
	}

	/// Allows synchronous burning of ESDT tokens. Execution is resumed afterwards.
	fn esdt_local_burn(&self, gas: u64, token: &[u8], amount: &Self::AmountType) {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(token);
		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.call_local_esdt_built_in_function(gas, b"ESDTLocalBurn", &arg_buffer);
	}

	/// Creates a new NFT token of a certain type (determined by `token_identifier`).  
	/// `attributes` can be any serializable custom struct.  
	/// This is a built-in function, so the smart contract execution is resumed after.
	fn esdt_nft_create<T: elrond_codec::TopEncode>(
		&self,
		gas: u64,
		token: &[u8],
		amount: &Self::AmountType,
		name: &BoxedBytes,
		royalties: &Self::AmountType,
		hash: &BoxedBytes,
		attributes: &T,
		uris: &[BoxedBytes],
	) {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(token);
		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());
		arg_buffer.push_argument_bytes(name.as_slice());
		arg_buffer.push_argument_bytes(royalties.to_bytes_be().as_slice());
		arg_buffer.push_argument_bytes(hash.as_slice());

		let mut top_encoded_attributes = Vec::new();
		let _ = attributes.top_encode(&mut top_encoded_attributes);
		arg_buffer.push_argument_bytes(top_encoded_attributes.as_slice());

		// The API function has the last argument as variadic,
		// so we top-encode each and send as separate argument
		for uri in uris {
			let mut top_encoded_uri = Vec::new();
			let _ = uri.top_encode(&mut top_encoded_uri);

			arg_buffer.push_argument_bytes(top_encoded_uri.as_slice());
		}

		self.call_local_esdt_built_in_function(gas, b"ESDTNFTCreate", &arg_buffer);
	}

	/// Adds quantity for an Non-Fungible Token. (which makes it a Semi-Fungible Token by definition)  
	/// This is a built-in function, so the smart contract execution is resumed after.
	fn esdt_nft_add_quantity(&self, gas: u64, token: &[u8], nonce: u64, amount: &Self::AmountType) {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(token);
		arg_buffer.push_argument_bytes(&nonce.to_be_bytes()[..]);
		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.call_local_esdt_built_in_function(gas, b"ESDTNFTAddQuantity", &arg_buffer);
	}

	/// The reverse operation of `esdt_nft_add_quantity`, this locally decreases
	/// This is a built-in function, so the smart contract execution is resumed after.
	fn esdt_nft_burn(&self, gas: u64, token: &[u8], nonce: u64, amount: &Self::AmountType) {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(token);
		arg_buffer.push_argument_bytes(&nonce.to_be_bytes()[..]);
		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.call_local_esdt_built_in_function(gas, b"ESDTNFTBurn", &arg_buffer);
	}

	/// Burns ESDT tokens. Handles any type of ESDT.
	/// Note: this does not work with EGLD, use only with ESDT.
	fn burn_tokens(
		&self,
		token: &TokenIdentifier,
		nonce: u64,
		amount: &Self::AmountType,
		gas: u64,
	) {
		if amount > &0 {
			if nonce == 0 {
				self.esdt_local_burn(gas, token.as_esdt_identifier(), amount);
			} else {
				self.esdt_nft_burn(gas, token.as_esdt_identifier(), nonce, amount);
			}
		}
	}

	/// Performs a simple ESDT NFT transfer, but via async call.
	/// This is the preferred way to send ESDT.
	/// Note: call is done to the SC itself, so `from` should be the SCs own address
	fn direct_esdt_nft_via_async_call(
		&self,
		from: &Address,
		to: &Address,
		token: &[u8],
		nonce: u64,
		amount: &Self::AmountType,
		data: &[u8],
	) {
		let mut serializer = HexCallDataSerializer::new(ESDT_NFT_TRANSFER_STRING);
		serializer.push_argument_bytes(token);
		serializer.push_argument_bytes(&nonce.to_be_bytes()[..]);
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
		serializer.push_argument_bytes(to.as_bytes());
		if !data.is_empty() {
			serializer.push_argument_bytes(data);
		}
		self.async_call_raw(&from, &Self::AmountType::zero(), serializer.as_slice());
	}

	/// Sends an ESDT NFT to a given address, directly.
	/// Used especially for sending ESDT to regular accounts.
	///
	/// Unlike sending ESDT via async call, this method can be called multiple times per transaction.
	fn direct_esdt_nft_via_transfer_exec(
		&self,
		to: &Address,
		token: &[u8],
		nonce: u64,
		amount: &Self::AmountType,
		data: &[u8],
	) -> Result<(), &'static [u8]> {
		self.direct_esdt_nft_execute(to, token, nonce, amount, 0, data, &ArgBuffer::new())
	}
}
