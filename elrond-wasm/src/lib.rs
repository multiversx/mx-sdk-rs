#![no_std]

// re-export basic heap types
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::string::String;
pub use alloc::vec::Vec;

pub use elrond_codec;

pub mod abi;
pub mod api;
pub mod err_msg;
pub mod hex_call_data;
pub mod io;
pub mod non_zero_util;
mod proxy;
pub mod storage;
pub mod types;

pub use hex_call_data::*;
pub use io::*;
pub use proxy::OtherContractHandle;
pub use storage::{storage_get, storage_set};
pub use types::*;

/// Handy way of casting to a contract proxy trait.
/// Would make more sense to be in elrond-wasm-derive, but Rust "cannot export macro_rules! macros from a `proc-macro` crate type currently".
#[macro_export]
macro_rules! contract_proxy {
	($s:expr, $address:expr, $proxy_trait:ident) => {
		$s.contract_proxy($address) as Box<dyn $proxy_trait<BigInt, BigUint>>
	};
}

/// Getting all imports needed for a smart contract.
#[macro_export]
macro_rules! imports {
	() => {
		use core::ops::{Add, Div, Mul, Rem, Sub};
		use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
		use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
		use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
		use elrond_wasm::api::{BigIntApi, BigUintApi, CallValueApi, ContractHookApi};
		use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
		use elrond_wasm::err_msg;
		use elrond_wasm::io::*;
		use elrond_wasm::non_zero_util::*;
		use elrond_wasm::storage::mappers::*;
		use elrond_wasm::types::*;
		use elrond_wasm::{AsyncCallError, AsyncCallResult, OtherContractHandle};
		use elrond_wasm::{BorrowedMutStorage, Box, BoxedBytes, Queue, VarArgs, Vec};
		use elrond_wasm::{SCError, SCResult, SCResult::Err, SCResult::Ok};
	};
}

/// Imports required for deriving serialization and TypeAbi.
#[macro_export]
macro_rules! derive_imports {
	() => {
		use elrond_wasm::elrond_codec;
		use elrond_wasm::elrond_codec::elrond_codec_derive::{
			NestedDecode, NestedEncode, TopDecode, TopEncode,
		};
		use elrond_wasm_derive::TypeAbi;
	};
}

/// Compact way of returning a static error message.
#[macro_export]
macro_rules! sc_error {
	($s:expr) => {
		elrond_wasm::SCResult::Err(elrond_wasm::SCError::from($s.as_bytes()))
	};
}

/// Equivalent of the ? operator for SCResult.
#[macro_export]
macro_rules! sc_try {
	($s:expr) => {
		match $s {
			elrond_wasm::SCResult::Ok(t) => t,
			elrond_wasm::SCResult::Err(e) => {
				return elrond_wasm::SCResult::Err(e);
				},
			}
	};
}

/// Allows us to write Solidity style `require!(<condition>, <error_msg>)` and avoid if statements.
///
/// It can only be used in a function that returns `SCResult<_>` where _ can be any type.
///
/// ```rust
/// # use elrond_wasm::{*, SCResult::Ok};
/// # pub trait ExampleContract<BigInt, BigUint>: elrond_wasm::api::ContractHookApi<BigInt, BigUint>
/// # where
/// # 	BigInt: elrond_wasm::api::BigIntApi<BigUint> + 'static,
/// # 	BigUint: elrond_wasm::api::BigUintApi + 'static,
/// # {
/// fn only_callable_by_owner(&self) -> SCResult<()> {
///     require!(self.get_caller() == self.get_owner_address(), "Caller must be owner");
///     Ok(())
/// }
/// # }
/// ```
#[macro_export]
macro_rules! require {
	($expression:expr, $error_msg:expr) => {
		if (!($expression)) {
			return sc_error!($error_msg);
			}
	};
}

/// Very compact way of not allowing anyone but the owner to call a function.
///
/// It can only be used in a function that returns `SCResult<_>` where _ can be any type.
///
/// ```rust
/// # use elrond_wasm::{*, SCResult::Ok};
/// # pub trait ExampleContract<BigInt, BigUint>: elrond_wasm::api::ContractHookApi<BigInt, BigUint>
/// # where
/// # 	BigInt: elrond_wasm::api::BigIntApi<BigUint> + 'static,
/// # 	BigUint: elrond_wasm::api::BigUintApi + 'static,
/// # {
/// fn only_callable_by_owner(&self) -> SCResult<()> {
///     only_owner!(self, "Caller must be owner");
///     Ok(())
/// }
/// # }
/// ```
#[macro_export]
macro_rules! only_owner {
	($trait_self: expr, $error_msg:expr) => {
		if ($trait_self.get_caller() != $trait_self.get_owner_address()) {
			return sc_error!($error_msg);
			}
	};
}

/// Compact way to represent the BorrowedMutStorage type.
#[macro_export]
macro_rules! mut_storage (
    ($t:ty) => (
        BorrowedMutStorage<T, $t>
    )
);

/// Converts usize to NonZeroUsize or returns SCError.
#[macro_export]
macro_rules! non_zero_usize {
	($input: expr, $error_msg:expr) => {
		if let Some(nz) = NonZeroUsize::new($input) {
				nz
		} else {
			return sc_error!($error_msg);
			}
	};
}
