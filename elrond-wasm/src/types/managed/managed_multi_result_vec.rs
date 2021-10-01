use super::{ManagedBuffer, ManagedType, ManagedVec, ManagedVecItem};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer},
    api::{EndpointFinishApi, ErrorApi, ManagedTypeApi},
    contract_base::ManagedSerializer,
    types::{ManagedArgBuffer, MultiResultVec},
    ArgId, ContractCallArg, DynArg, DynArgInput, DynArgOutput, EndpointResult,
};
use alloc::string::String;
use core::marker::PhantomData;
use elrond_codec::{try_cast_execute_or_else, TopDecode, TopEncode};

#[derive(Clone)]
pub struct ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    pub(super) raw_buffers: ManagedVec<M, ManagedBuffer<M>>,
    _phantom: PhantomData<T>,
}

impl<M, T> ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn from_raw_vec(raw_buffers: ManagedVec<M, ManagedBuffer<M>>) -> Self {
        ManagedMultiResultVec {
            raw_buffers,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn new(api: M) -> Self {
        ManagedMultiResultVec::from_raw_vec(ManagedVec::new(api))
    }
}

impl<M, T> ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncode,
{
    pub fn push(&mut self, item: T) {
        let serializer = ManagedSerializer::new(self.raw_buffers.type_manager());
        self.raw_buffers
            .push(serializer.top_encode_to_managed_buffer(&item));
    }
}

impl<M, T> From<ManagedVec<M, T>> for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M> + TopEncode + 'static,
{
    #[inline]
    #[rustfmt::skip]
    fn from(v: ManagedVec<M, T>) -> Self {
        try_cast_execute_or_else(
            v,
            ManagedMultiResultVec::from_raw_vec,
            |v| ManagedMultiResultVec::from(&v),
        )
    }
}

impl<M, T> From<&ManagedVec<M, T>> for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M> + TopEncode,
{
    #[inline]
    fn from(v: &ManagedVec<M, T>) -> Self {
        let mut result = ManagedMultiResultVec::new(v.type_manager());
        for item in v.into_iter() {
            result.push(item);
        }
        result
    }
}

impl<M, T> ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    pub fn to_arg_buffer(&self) -> ManagedArgBuffer<M> {
        ManagedArgBuffer::from_raw_handle(
            self.raw_buffers.type_manager(),
            self.raw_buffers.get_raw_handle(),
        )
    }
}

impl<M> ManagedMultiResultVec<M, ManagedBuffer<M>>
where
    M: ManagedTypeApi,
{
    pub fn into_vec_of_buffers(self) -> ManagedVec<M, ManagedBuffer<M>> {
        self.raw_buffers
    }
}

impl<M, T> ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: ManagedVecItem<M> + TopDecode,
{
    pub fn to_vec(&self) -> ManagedVec<M, T> {
        let mut result = ManagedVec::new(self.raw_buffers.type_manager());
        let serializer = ManagedSerializer::new(self.raw_buffers.type_manager());
        for item in self.raw_buffers.into_iter() {
            result.push(serializer.top_decode_from_managed_buffer(&item));
        }
        result
    }
}

impl<M, T> DynArg for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    // #[inline(never)]
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        let mut raw_buffers = ManagedVec::new(loader.vm_api_cast::<M>());
        while loader.has_next() {
            raw_buffers.push(ManagedBuffer::dyn_load(loader, arg_id));
        }
        ManagedMultiResultVec {
            raw_buffers,
            _phantom: PhantomData,
        }
    }
}

impl<M, T> EndpointResult for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: EndpointResult,
{
    type DecodeAs = ManagedMultiResultVec<M, T::DecodeAs>;

    #[inline]
    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        for elem in self.raw_buffers.into_iter() {
            elem.finish(api.clone());
        }
    }
}

impl<M, T> ContractCallArg for &ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        for elem in self.raw_buffers.into_iter() {
            elem.push_dyn_arg(output);
        }
    }
}

impl<M, T> ContractCallArg for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
    }
}

impl<M, T> TypeAbi for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi,
{
    fn type_name() -> String {
        MultiResultVec::<T>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_multi_arg_or_result() -> bool {
        true
    }
}
