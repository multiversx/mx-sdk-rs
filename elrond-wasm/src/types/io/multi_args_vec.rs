use crate::abi::{TypeAbi, TypeDescriptionContainer};
use crate::io::{ArgId, ContractCallArg, DynArg, DynArgInput};
use crate::types::{ArgBuffer, SCError};
use crate::{api::EndpointFinishApi, EndpointResult};
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::FromIterator;
use elrond_codec::TopDecodeInput;

/// Structure that allows taking a variable number of arguments
/// or returning a variable number of results in a smart contract endpoint.
#[derive(Clone)]
pub struct MultiArgVec<T>(pub Vec<T>);

/// Used for taking a variable number of arguments in an endpoint,
/// it is synonymous with `MultiResultVec`/`MultiArgVec`.
pub type VarArgs<T> = MultiArgVec<T>;

/// Used for returning a variable number of results from an endpoint,
/// it is synonymous with `MultiResult`.
pub type MultiResultVec<T> = VarArgs<T>;

impl<T> From<Vec<T>> for MultiArgVec<T> {
    fn from(v: Vec<T>) -> Self {
        MultiArgVec(v)
    }
}

impl<T> MultiArgVec<T> {
    #[inline]
    pub fn new() -> Self {
        MultiArgVec(Vec::new())
    }
}

impl<T> Default for MultiArgVec<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MultiArgVec<T> {
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> FromIterator<T> for MultiArgVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let v = Vec::<T>::from_iter(iter);
        MultiArgVec(v)
    }
}

impl<T> DynArg for MultiArgVec<T>
where
    T: DynArg,
{
    // #[inline(never)]
    fn dyn_load<I, D>(loader: &mut D, arg_id: ArgId) -> Self
    where
        I: TopDecodeInput,
        D: DynArgInput<I>,
    {
        let mut result_vec: Vec<T> = Vec::new();
        while loader.has_next() {
            result_vec.push(T::dyn_load(loader, arg_id));
        }
        MultiArgVec(result_vec)
    }
}

impl<T> EndpointResult for MultiArgVec<T>
where
    T: EndpointResult,
{
    type DecodeAs = MultiArgVec<T::DecodeAs>;

    #[inline]
    fn finish<FA>(&self, api: FA)
    where
        FA: EndpointFinishApi + Clone + 'static,
    {
        for elem in self.0.iter() {
            elem.finish(api.clone());
        }
    }
}

impl<T> ContractCallArg for &MultiArgVec<T>
where
    T: ContractCallArg,
{
    fn push_async_arg(&self, serializer: &mut ArgBuffer) -> Result<(), SCError> {
        for elem in self.0.iter() {
            elem.push_async_arg(serializer)?;
        }
        Ok(())
    }
}

impl<T> ContractCallArg for MultiArgVec<T>
where
    T: ContractCallArg,
{
    fn push_async_arg(&self, serializer: &mut ArgBuffer) -> Result<(), SCError> {
        (&self).push_async_arg(serializer)
    }
}

impl<T: TypeAbi> TypeAbi for MultiArgVec<T> {
    fn type_name() -> String {
        let mut repr = String::from("variadic<");
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
