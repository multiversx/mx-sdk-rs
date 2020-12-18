use super::sc_error::SCError;
use crate::abi::{OutputAbi, TypeAbi, TypeDescription};
use crate::io::finish::EndpointResult;
use crate::*;

/// Default way to optionally return an error from a smart contract endpoint.
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub enum SCResult<T> {
	Ok(T),
	Err(SCError),
}

impl<T> SCResult<T> {
	#[inline]
	pub fn is_ok(&self) -> bool {
		if let SCResult::Ok(_) = self {
			true
		} else {
			false
		}
	}

	#[inline]
	pub fn is_err(&self) -> bool {
		!self.is_ok()
	}

	#[inline]
	pub fn ok(self) -> Option<T> {
		if let SCResult::Ok(t) = self {
			Some(t)
		} else {
			None
		}
	}

	#[inline]
	pub fn err(self) -> Option<SCError> {
		if let SCResult::Err(e) = self {
			Some(e)
		} else {
			None
		}
	}
}

impl<A, BigInt, BigUint, T> EndpointResult<A, BigInt, BigUint> for SCResult<T>
where
	T: EndpointResult<A, BigInt, BigUint>,
	BigInt: BigIntApi<BigUint> + 'static,
	BigUint: BigUintApi + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
{
	#[inline]
	fn finish(&self, api: A) {
		match self {
			SCResult::Ok(t) => {
				t.finish(api);
			},
			SCResult::Err(e) => {
				api.signal_error(e.as_bytes());
			},
		}
	}
}

impl<T: TypeAbi> TypeAbi for SCResult<T> {
	fn type_name() -> String {
		T::type_name()
	}

	fn output_abis() -> Vec<OutputAbi> {
		T::output_abis()
	}

	fn type_description() -> TypeDescription {
		T::type_description()
	}
}

impl<T> SCResult<T> {
	pub fn unwrap(self) -> T {
		match self {
			SCResult::Ok(t) => t,
			SCResult::Err(_) => panic!("called `SCResult::unwrap()`"),
		}
	}
}
