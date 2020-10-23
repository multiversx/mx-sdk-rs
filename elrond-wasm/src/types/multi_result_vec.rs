use alloc::vec::Vec;
use core::iter::FromIterator;
use crate::{EndpointResult, ContractHookApi, ContractIOApi, BigIntApi, BigUintApi};

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
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let v = Vec::<T>::from_iter(iter);
        MultiResultVec(v)
    }
}

impl<'a, A, BigInt, BigUint, T> EndpointResult<'a, A, BigInt, BigUint> for MultiResultVec<T>
where
    T: EndpointResult<'a, A, BigInt, BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    BigUint: BigUintApi + 'static,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'a
{
    #[inline]
    fn finish(&self, api: &'a A) {
        for elem in self.0.iter() {
            elem.finish(api);
        }
    }
}
