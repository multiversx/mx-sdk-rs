use crate::abi::{ContractAbi, TypeAbi};
use alloc::string::String;

/// Required by contract ABI generators.
/// Provides the same associated types as the `ContractBase`,
/// so that associated types that show up in arguments and results match.
pub trait ContractAbiProvider {
	/// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
	/// It sometimes references the contract storage manager type in with storage mappers,
	/// as for example in `SingleValueMapper<Self::Storage, i32>`.
	type Storage;

	/// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
	/// This associated type allows `Self::BigUint` to also make sense in the ABI context.
	type BigUint: TypeAbi;

	/// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
	/// This associated type allows `Self::BigInt` to also make sense in the ABI context.
	type BigInt: TypeAbi;

	/// Associated function that provides the contract or module ABI.
	/// Since ABI generation is static, no state from the contract is required.
	fn abi() -> ContractAbi;
}

/// Dummy type with no implementation.
/// Provides context in ABI generators.
pub struct StorageAbiOnly;

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
