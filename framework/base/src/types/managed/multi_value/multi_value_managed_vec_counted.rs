use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
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
pub struct MultiValueManagedVecCounted<'a, M, T>
where
    M: ManagedTypeApi<'a>,
    T: ManagedVecItem,
{
    pub(super) contents: ManagedVec<'a, M, T>,
}

#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueManagedVecCounted`"
)]
pub type ManagedCountedVarArgs<'a, M, T> = MultiValueManagedVecCounted<'a, M, T>;

#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueManagedVecCounted`"
)]
pub type ManagedCountedMultiResultVec<'a, M, T> = MultiValueManagedVecCounted<'a, M, T>;

impl<'a, M, T> MultiValueManagedVecCounted<'a, M, T>
where
    M: ManagedTypeApi<'a>,
    T: ManagedVecItem,
{
    #[inline]
    pub fn new() -> Self {
        MultiValueManagedVecCounted::from(ManagedVec::new())
    }
}

impl<'a, M, T> MultiValueManagedVecCounted<'a, M, T>
where
    M: ManagedTypeApi<'a>,
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

impl<'a, M, T> MultiValueManagedVecCounted<'a, M, T>
where
    M: ManagedTypeApi<'a>,
    T: ManagedVecItem,
{
    #[inline]
    pub fn push(&mut self, item: T) {
        self.contents.push(item);
    }

    #[inline]
    pub fn into_vec(self) -> ManagedVec<'a, M, T> {
        self.contents
    }
}

impl<'a, M, T> From<ManagedVec<'a, M, T>> for MultiValueManagedVecCounted<'a, M, T>
where
    M: ManagedTypeApi<'a>,
    T: ManagedVecItem,
{
    #[inline]
    #[rustfmt::skip]
    fn from(v: ManagedVec<'a, M, T>) -> Self {
        MultiValueManagedVecCounted {
            contents: v,
        }
    }
}

impl<'a, M, T> TopEncodeMulti for MultiValueManagedVecCounted<'a, M, T>
where
    M: ManagedTypeApi<'a>,
    T: ManagedVecItem + TopEncodeMulti,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        self.len().multi_encode_or_handle_err(output, h)?;
        for elem in self.contents.into_iter() {
            elem.multi_encode_or_handle_err(output, h)?;
        }
        Ok(())
    }
}

impl<'a, M, T> TopDecodeMulti for MultiValueManagedVecCounted<'a, M, T>
where
    M: ManagedTypeApi<'a>,
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

impl<'a, M, T> TypeAbi for MultiValueManagedVecCounted<'a, M, T>
where
    M: ManagedTypeApi<'a>,
    T: ManagedVecItem + TypeAbi,
{
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
