use core::borrow::Borrow;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName},
    api::ManagedTypeApi,
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput,
        TopEncodeMulti, TopEncodeMultiOutput,
    },
    types::{ManagedVec, ManagedVecItem},
};

/// Argument or result that is made up of the argument count, followed by the arguments themselves.
/// Think of it as a `VarArgs` preceded by the count.
/// Unlike `MultiValueManagedVec` it deserializes eagerly.
#[derive(Clone, Default)]
pub struct MultiValueManagedVecCounted<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    pub(super) contents: ManagedVec<M, T>,
}

#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueManagedVecCounted`"
)]
pub type ManagedCountedVarArgs<M, T> = MultiValueManagedVecCounted<M, T>;

#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueManagedVecCounted`"
)]
pub type ManagedCountedMultiResultVec<M, T> = MultiValueManagedVecCounted<M, T>;

impl<M, T> MultiValueManagedVecCounted<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    #[inline]
    pub fn new() -> Self {
        MultiValueManagedVecCounted::from(ManagedVec::new())
    }
}

impl<M, T> MultiValueManagedVecCounted<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
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

impl<M, T> MultiValueManagedVecCounted<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
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

impl<M, T> From<ManagedVec<M, T>> for MultiValueManagedVecCounted<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    #[inline]
    #[rustfmt::skip]
    fn from(v: ManagedVec<M, T>) -> Self {
        MultiValueManagedVecCounted {
            contents: v,
        }
    }
}

impl<M, T> ManagedVecItem for MultiValueManagedVecCounted<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    type PAYLOAD = <ManagedVec<M, T> as ManagedVecItem>::PAYLOAD;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = Self;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        Self::from(ManagedVec::<M, T>::read_from_payload(payload))
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        Self::read_from_payload(payload)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        self.contents.save_to_payload(payload);
    }
}

impl<M, T> TopEncodeMulti for MultiValueManagedVecCounted<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TopEncodeMulti,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        self.len().multi_encode_or_handle_err(output, h)?;
        for elem in &self.contents {
            elem.borrow().multi_encode_or_handle_err(output, h)?;
        }
        Ok(())
    }
}

impl<M, T> TopDecodeMulti for MultiValueManagedVecCounted<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TopDecodeMulti,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let count = usize::multi_decode_or_handle_err(input, h)?;
        let mut result = MultiValueManagedVecCounted::new();
        for _ in 0..count {
            result.push(T::multi_decode_or_handle_err(input, h)?);
        }
        Ok(result)
    }
}

impl<M, T> TypeAbiFrom<Self> for MultiValueManagedVecCounted<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TypeAbi,
{
}

impl<M, T> TypeAbi for MultiValueManagedVecCounted<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + TypeAbi,
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        let mut repr = TypeName::from("counted-variadic<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}
