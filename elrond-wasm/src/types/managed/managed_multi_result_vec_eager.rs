use alloc::string::String;
use elrond_codec::Vec;

use crate::{
    abi::{TypeAbi, TypeDescriptionContainer},
    api::{EndpointFinishApi, ManagedTypeApi},
    ArgId, ContractCallArg, DynArg, DynArgInput, DynArgOutput, EndpointResult,
};

use super::{ManagedVec, ManagedVecItem, ManagedVecIterator};

#[derive(Clone, Default)]
pub struct ManagedMultiResultVecEager<M: ManagedTypeApi, T: ManagedVecItem>(ManagedVec<M, T>);

pub type ManagedVarArgsEager<M, T> = ManagedMultiResultVecEager<M, T>;

impl<M, T> From<ManagedVec<M, T>> for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    #[inline]
    fn from(managed_vec: ManagedVec<M, T>) -> Self {
        ManagedMultiResultVecEager(managed_vec)
    }
}

impl<M, T> ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    #[inline]
    pub fn new() -> Self {
        ManagedMultiResultVecEager(ManagedVec::new())
    }

    #[inline]
    pub fn byte_len(&self) -> usize {
        self.0.byte_len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<T> {
        self.0.get(index)
    }

    pub fn slice(&self, start_index: usize, end_index: usize) -> Option<Self> {
        self.0
            .slice(start_index, end_index)
            .map(|value| Self(value))
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item)
    }

    pub fn from_single_item(item: T) -> Self {
        let mut result = ManagedMultiResultVecEager::new();
        result.push(item);
        result
    }

    pub fn overwrite_with_single_item(&mut self, item: T) {
        self.0.overwrite_with_single_item(item)
    }

    pub fn append_vec(&mut self, item: ManagedMultiResultVecEager<M, T>) {
        self.0.append_vec(item.0)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn into_vec(self) -> ManagedVec<M, T> {
        self.0
    }

    pub fn with_self_as_vec<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Vec<T>),
    {
        self.0.with_self_as_vec(f)
    }

    pub fn iter(&self) -> ManagedVecIterator<M, T> {
        ManagedVecIterator::new(&self.0)
    }
}

impl<M, T, I> From<Vec<I>> for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
    I: Into<T>,
{
    fn from(v: Vec<I>) -> Self {
        let mut result = Self::new();
        for item in v.into_iter() {
            result.push(item.into());
        }
        result
    }
}
impl<M, T> DynArg for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + DynArg,
{
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        let mut result_vec: ManagedVec<M, T> = ManagedVec::new();
        while loader.has_next() {
            result_vec.push(T::dyn_load(loader, arg_id));
        }
        ManagedMultiResultVecEager(result_vec)
    }
}

impl<M, T> EndpointResult for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + EndpointResult,
{
    type DecodeAs = ManagedMultiResultVecEager<M, T>;

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
impl<M, T> ContractCallArg for &ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        for elem in self.0.iter() {
            elem.push_dyn_arg(output);
        }
    }
}

impl<M, T> ContractCallArg for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
    }
}

impl<M, T: TypeAbi> TypeAbi for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
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
