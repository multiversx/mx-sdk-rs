use crate::abi;

/// ContractWithAbi is the means by which a contract can provide an ABI.
pub trait ContractWithAbi {
	/// The generated ABI generation code sometimes references the contract storage manager type,
	/// e.g. with storage mappers.
	type Storage;

	/// Generate a raw ABI object.
	/// Contracts would not call this function, so it never ends up in the wasm bytecode.
	/// It is, however, still no_std.
	fn abi(&self, include_modules: bool) -> abi::ContractAbi;
}
