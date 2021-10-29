use super::{ManagedBuffer, ManagedMultiResultVec, ManagedMultiResultVecIterator, ManagedVec};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer},
    api::{EndpointFinishApi, ErrorApi, ManagedTypeApi},
    ArgId, ContractCallArg, DynArg, DynArgInput, DynArgOutput, EndpointResult,
};
use alloc::string::String;
use core::marker::PhantomData;
use elrond_codec::TopEncode;

/// Argument or result that is made up of the argument count, followed by the arguments themselves.
/// Think of it as a `VarArgs` preceded by the count.
pub struct ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    pub(super) contents: ManagedMultiResultVec<M, T>,
    _phantom: PhantomData<T>,
}

pub type ManagedCountedVarArgs<M, T> = ManagedCountedMultiResultVec<M, T>;

impl<M, T> ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    #[inline]
    pub fn new(api: M) -> Self {
        ManagedCountedMultiResultVec::from(ManagedMultiResultVec::new(api))
    }
}

impl<M, T> ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
{
    #[inline]
    pub fn len(&self) -> usize {
        self.contents.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }
}

impl<M, T> ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncode,
{
    #[inline]
    pub fn push(&mut self, item: T) {
        self.contents.push(item);
    }
}

impl<M, T> From<ManagedMultiResultVec<M, T>> for ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    #[inline]
    #[rustfmt::skip]
    fn from(v: ManagedMultiResultVec<M, T>) -> Self {
        ManagedCountedMultiResultVec {
            contents: v,
            _phantom: PhantomData,
        }
    }
}

impl<M, T> IntoIterator for ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: DynArg,
{
    type Item = T;
    type IntoIter = ManagedMultiResultVecIterator<M, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

impl<M> ManagedCountedMultiResultVec<M, ManagedBuffer<M>>
where
    M: ManagedTypeApi,
{
    pub fn into_vec_of_buffers(self) -> ManagedVec<M, ManagedBuffer<M>> {
        self.contents.into_vec_of_buffers()
    }
}

impl<M, T> DynArg for ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    // #[inline(never)]
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        let mut result = ManagedCountedMultiResultVec::new(loader.vm_api_cast::<M>());
        let count = usize::dyn_load(loader, arg_id);
        for _ in 0..count {
            result
                .contents
                .raw_buffers
                .push(ManagedBuffer::dyn_load(loader, arg_id));
        }
        result
    }
}

impl<M, T> EndpointResult for ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: EndpointResult,
{
    type DecodeAs = ManagedCountedMultiResultVec<M, T::DecodeAs>;

    #[inline]
    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        self.len().finish(api.clone());
        self.contents.finish(api);
    }
}

impl<M, T> ContractCallArg for &ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        self.len().push_dyn_arg(output);
        self.contents.push_dyn_arg(output);
    }
}

impl<M, T> ContractCallArg for ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
    }
}

impl<M, T> TypeAbi for ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi,
{
    fn type_name() -> String {
        let mut repr = String::from("counted-variadic<");
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
