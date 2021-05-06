use super::{ErrorApi, StorageReadApi, StorageWriteApi};
use crate::abi::{ContractAbi, TypeAbi};
use alloc::string::String;
use alloc::vec::Vec;

/// Required by contract ABI generators.
/// Provides the same associated types as the `ContractBase`,
/// so that associated types that show up in arguments and results match.
pub trait ContractAbiProvider {
	/// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
	/// This associated type allows `Self::BigUint` to also make sense in the ABI context.
	type BigUint: TypeAbi;

	/// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
	/// This associated type allows `Self::BigInt` to also make sense in the ABI context.
	type BigInt: TypeAbi;

	/// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
	/// It sometimes references the contract storage manager type in with storage mappers,
	/// as for example in `SingleValueMapper<Self::Storage, i32>`.
	type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

	/// Associated function that provides the contract or module ABI.
	/// Since ABI generation is static, no state from the contract is required.
	fn abi() -> ContractAbi;
}

/// Dummy type with no implementation, just the ABI.
/// Provides context in ABI generators.
pub struct BigUintAbiOnly;

impl TypeAbi for BigUintAbiOnly {
	fn type_name() -> String {
		String::from("BigUint")
	}
}

/// Dummy type with no implementation, just the ABI.
/// Provides context in ABI generators.
pub struct BigIntAbiOnly;

impl TypeAbi for BigIntAbiOnly {
	fn type_name() -> String {
		String::from("BigInt")
	}
}

/// Dummy type with no implementation.
/// Provides context in ABI generators.
#[derive(Clone)]
pub struct StorageAbiOnly;

impl StorageReadApi for StorageAbiOnly {
	fn storage_load_len(&self, _key: &[u8]) -> usize {
		0
	}

	fn storage_load_vec_u8(&self, _key: &[u8]) -> Vec<u8> {
		Vec::new()
	}

	fn storage_load_big_uint_raw(&self, _key: &[u8]) -> i32 {
		0
	}

	fn storage_load_u64(&self, _key: &[u8]) -> u64 {
		0
	}

	fn storage_load_i64(&self, _key: &[u8]) -> i64 {
		0
	}
}

impl StorageWriteApi for StorageAbiOnly {
	fn storage_store_slice_u8(&self, _key: &[u8], _value: &[u8]) {}

	fn storage_store_big_uint_raw(&self, _key: &[u8], _handle: i32) {}

	fn storage_store_u64(&self, _key: &[u8], _value: u64) {}

	fn storage_store_i64(&self, _key: &[u8], _value: i64) {}
}

impl ErrorApi for StorageAbiOnly {
	fn signal_error(&self, _message: &[u8]) -> ! {
		unreachable!()
	}
}
