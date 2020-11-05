use crate::{BigIntApi, BigUintApi, ContractHookApi, ContractIOApi, EndpointResult};

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
