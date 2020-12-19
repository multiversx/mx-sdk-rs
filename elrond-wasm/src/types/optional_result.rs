use crate::abi::{TypeAbi, TypeDescriptionContainer};
use crate::{BigIntApi, BigUintApi, ContractHookApi, ContractIOApi, EndpointResult};
use alloc::string::String;

pub enum OptionalResult<T> {
	Some(T),
	None,
}

impl<T> From<Option<T>> for OptionalResult<T> {
	fn from(v: Option<T>) -> Self {
		match v {
			Some(result) => OptionalResult::Some(result),
			None => OptionalResult::None,
		}
	}
}

impl<A, BigInt, BigUint, T> EndpointResult<A, BigInt, BigUint> for OptionalResult<T>
where
	T: EndpointResult<A, BigInt, BigUint>,
	BigInt: BigIntApi<BigUint> + 'static,
	BigUint: BigUintApi + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
{
	#[inline]
	fn finish(&self, api: A) {
		if let OptionalResult::Some(t) = self {
			t.finish(api);
		}
	}
}

impl<T: TypeAbi> TypeAbi for OptionalResult<T> {
	fn type_name() -> String {
		let mut repr = String::from("OptionalResult<");
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
