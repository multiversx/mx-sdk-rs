use super::{
    ManagedBuffer, ManagedDefault, ManagedFrom, ManagedInto, ManagedType, ManagedVec,
    ManagedVecItem,
};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer},
    api::{EndpointFinishApi, Handle, ManagedTypeApi},
    types::{ArgBuffer, BoxedBytes, ManagedBufferNestedDecodeInput, MultiArgVec},
    ArgId, ContractCallArg, DynArg, DynArgInput, DynArgOutput, EndpointResult,
};
use alloc::string::String;
use core::{any::Any, iter::FromIterator, marker::PhantomData, ops::Deref, result};
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, Vec,
};

pub struct ManagedMultiResultVec<M, T>(pub ManagedVec<M, T>)
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>;

impl<M, T> Deref for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    type Target = ManagedVec<M, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M, T> ManagedType<M> for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    fn from_raw_handle(api: M, raw_handle: Handle) -> Self {
        Self(ManagedVec::from_raw_handle(api, raw_handle))
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.buffer.handle
    }

    #[inline]
    fn type_manager(&self) -> M {
        self.buffer.api.clone()
    }
}

impl<M, T> ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    pub(crate) fn new_from_raw_buffer(buffer: ManagedBuffer<M>) -> Self {
        Self(ManagedVec::new_from_raw_buffer(buffer))
    }
}

impl<M, T> From<ManagedVec<M, T>> for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    fn from(b: ManagedVec<M, T>) -> Self {
        Self(b)
    }
}

impl<M, T> ManagedDefault<M> for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    fn managed_default(api: M) -> Self {
        Self(ManagedVec::managed_default(api))
    }
}

impl<M, T> ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    /// Length of the underlying buffer in bytes.

    pub fn slice(&self, start_index: usize, end_index: usize) -> Option<Self> {
        self.0.slice(start_index, end_index).map(Self)
    }

    pub fn append_vec(&mut self, item: ManagedMultiResultVec<M, T>) {
        self.0.append_vec(item.0);
    }
}

impl<M, T> DynArg for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: DynArg + ManagedVecItem<M>,
{
    // #[inline(never)]
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        let mut result_vec: Vec<T> = Vec::new();
        while loader.has_next() {
            result_vec.push(T::dyn_load(loader, arg_id));
        }
        Self(ManagedVec::managed_from(loader.error_api(), result_vec))
    }
}

impl<M, T> EndpointResult for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: EndpointResult + ManagedVecItem<M>,
    <T as EndpointResult>::DecodeAs: ManagedVecItem<M>,
{
    type DecodeAs = ManagedMultiResultVec<M, T::DecodeAs>;

    #[inline]
    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        for elem in self.0.into_vec().iter() {
            elem.finish(api.clone());
        }
    }
}

impl<M, T> ContractCallArg for &ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ContractCallArg + ManagedVecItem<M>,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        for elem in self.0.into_vec().iter() {
            elem.push_dyn_arg(output);
        }
    }
}

impl<M, T> ContractCallArg for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ContractCallArg + ManagedVecItem<M>,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
    }
}

impl<M, T: TypeAbi> TypeAbi for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
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
