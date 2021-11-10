use super::{ManagedVec, ManagedVecItem};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer},
    api::{EndpointFinishApi, ManagedTypeApi},
    finish_all, ArgId, ContractCallArg, DynArg, DynArgInput, DynArgOutput, EndpointResult,
};
use alloc::string::String;

/// Argument or result that is made up of the argument count, followed by the arguments themselves.
/// Think of it as a `VarArgs` preceded by the count.
/// Unlike `ManagedMultiResultVec` it deserializes eagerly.
pub struct ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    pub(super) contents: ManagedVec<M, T>,
}

pub type ManagedCountedVarArgs<M, T> = ManagedCountedMultiResultVec<M, T>;

impl<M, T> ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    pub fn new(api: M) -> Self {
        ManagedCountedMultiResultVec::from(ManagedVec::new(api))
    }
}

impl<M, T> ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
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
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    pub fn push(&mut self, item: T) {
        self.contents.push(item);
    }

    #[inline]
    pub fn into_vec(self) -> ManagedVec<M, T> {
        self.contents
    }
}

impl<M, T> From<ManagedVec<M, T>> for ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    #[rustfmt::skip]
    fn from(v: ManagedVec<M, T>) -> Self {
        ManagedCountedMultiResultVec {
            contents: v,
        }
    }
}

impl<M, T> DynArg for ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M> + DynArg,
{
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        let mut result = ManagedCountedMultiResultVec::new(loader.vm_api_cast::<M>());
        let count = usize::dyn_load(loader, arg_id);
        for _ in 0..count {
            result.contents.push(T::dyn_load(loader, arg_id));
        }
        result
    }
}

impl<M, T> EndpointResult for ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M> + EndpointResult,
{
    type DecodeAs = ManagedCountedMultiResultVec<M, T>;

    #[inline]
    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        self.len().finish(api.clone());
        finish_all(api, self.contents.iter());
    }
}

impl<M, T> ContractCallArg for &ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M> + ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        self.len().push_dyn_arg(output);
        for item in self.contents.iter() {
            item.push_dyn_arg(output);
        }
    }
}

impl<M, T> ContractCallArg for ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M> + ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
    }
}

impl<M, T> TypeAbi for ManagedCountedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M> + TypeAbi,
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
