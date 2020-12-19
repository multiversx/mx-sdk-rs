use crate::abi::{TypeAbi, TypeDescriptionContainer};
use crate::{BigIntApi, BigUintApi, ContractHookApi, ContractIOApi, EndpointResult};
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::FromIterator;

/// Structure that allows returning a variable number of results from a smart contract.
pub struct MultiResultVec<T>(pub Vec<T>);

impl<T> MultiResultVec<T> {
	#[inline]
	pub fn new() -> Self {
		MultiResultVec(Vec::new())
	}
}

impl<T> Default for MultiResultVec<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> From<Vec<T>> for MultiResultVec<T> {
	fn from(v: Vec<T>) -> Self {
		MultiResultVec(v)
	}
}

impl<T> FromIterator<T> for MultiResultVec<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let v = Vec::<T>::from_iter(iter);
		MultiResultVec(v)
	}
}

impl<A, BigInt, BigUint, T> EndpointResult<A, BigInt, BigUint> for MultiResultVec<T>
where
	T: EndpointResult<A, BigInt, BigUint>,
	BigInt: BigIntApi<BigUint> + 'static,
	BigUint: BigUintApi + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
{
	#[inline]
	fn finish(&self, api: A) {
		for elem in self.0.iter() {
			elem.finish(api.clone());
		}
	}
}

impl<T: TypeAbi> TypeAbi for MultiResultVec<T> {
	fn type_name() -> String {
		let mut repr = String::from("MultiResultVec<");
		repr.push_str(T::type_name().as_str());
		repr.push('>');
		repr
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
		T::provide_type_descriptions(accumulator);
	}

	fn is_multi_arg_or_result() -> bool {
		true
	}
}
