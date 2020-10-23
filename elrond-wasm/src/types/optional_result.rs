use crate::{EndpointResult, ContractHookApi, ContractIOApi, BigIntApi, BigUintApi};

pub enum OptionalResult<T> {
    Some(T),
    None
}

impl<T> From<Option<T>> for OptionalResult<T> {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(result) => OptionalResult::Some(result),
            None => OptionalResult::None,
        }
    }
}

impl<'a, A, BigInt, BigUint, T> EndpointResult<'a, A, BigInt, BigUint> for OptionalResult<T>
where
    T: EndpointResult<'a, A, BigInt, BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    #[inline]
    fn finish(&self, api: &'a A) {
        if let OptionalResult::Some(t) = self {
            t.finish(api);
        }
    }
}
