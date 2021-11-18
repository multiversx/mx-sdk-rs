use crate::{
    abi::{TypeAbi, TypeDescriptionContainer},
    api::{EndpointFinishApi, ManagedTypeApi},
    io::{ArgId, ContractCallArg, DynArg, DynArgInput},
    DynArgOutput, EndpointResult,
};
use alloc::{string::String, vec::Vec};
use core::iter::FromIterator;

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
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
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
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
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
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        for elem in self.0.iter() {
            elem.push_dyn_arg(output);
        }
    }
}

impl<T> ContractCallArg for MultiArgVec<T>
where
    T: ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
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
