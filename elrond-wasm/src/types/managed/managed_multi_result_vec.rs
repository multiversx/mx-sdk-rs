use super::{ManagedBuffer, ManagedType, ManagedVec, ManagedVecItem};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer},
    api::{ErrorApi, ManagedTypeApi},
    contract_base::{ExitCodecErrorHandler, ManagedSerializer},
    err_msg,
    types::{ManagedArgBuffer, MultiResultVec},
};
use alloc::string::String;
use core::marker::PhantomData;
use elrond_codec::{
    try_cast_execute_or_else, DecodeErrorHandler, EncodeErrorHandler, TopDecode, TopDecodeMulti,
    TopDecodeMultiInput, TopEncode, TopEncodeMulti, TopEncodeMultiOutput,
};

#[derive(Clone, Default)]
pub struct ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
{
    pub(super) raw_buffers: ManagedVec<M, ManagedBuffer<M>>,
    _phantom: PhantomData<T>,
}

pub type ManagedVarArgs<M, T> = ManagedMultiResultVec<M, T>;

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
    pub fn new() -> Self {
        ManagedMultiResultVec::from_raw_vec(ManagedVec::new())
    }
}

impl<M, T> ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti,
{
    pub fn push(&mut self, item: T) {
        let Ok(()) = item.multi_encode_or_handle_err(
            &mut self.raw_buffers,
            ExitCodecErrorHandler::<M>::from(err_msg::SERIALIZER_ENCODE_ERROR),
        );
    }
}

impl<M, T> From<ManagedVec<M, T>> for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TopEncode + 'static,
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
    T: ManagedVecItem + TopEncode,
{
    #[inline]
    fn from(v: &ManagedVec<M, T>) -> Self {
        let mut result = ManagedMultiResultVec::new();
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
        ManagedArgBuffer::from_raw_handle(self.raw_buffers.get_raw_handle())
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
{
    pub fn len(&self) -> usize {
        self.raw_buffers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.raw_buffers.is_empty()
    }
}

impl<M, T> ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: ManagedVecItem + TopDecode,
{
    pub fn to_vec(&self) -> ManagedVec<M, T> {
        let mut result = ManagedVec::new();
        let serializer = ManagedSerializer::<M>::new();
        for item in self.raw_buffers.into_iter() {
            result.push(serializer.top_decode_from_managed_buffer(&item));
        }
        result
    }
}

impl<M, T> TopEncodeMulti for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopEncodeMulti,
{
    type DecodeAs = Self;

    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        for elem in self.raw_buffers.into_iter() {
            elem.multi_encode_or_handle_err(output, h)?;
        }
        Ok(())
    }
}

impl<M, T> TopDecodeMulti for ManagedMultiResultVec<M, T>
where
    M: ManagedTypeApi + ErrorApi,
    T: TopDecodeMulti,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let mut raw_buffers = ManagedVec::new();
        while input.has_next() {
            raw_buffers.push(input.next_value(h)?);
        }
        Ok(Self {
            raw_buffers,
            _phantom: PhantomData,
        })
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

    fn is_variadic() -> bool {
        true
    }
}
