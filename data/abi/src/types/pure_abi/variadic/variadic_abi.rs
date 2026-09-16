use crate::{
    TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName, codec::multi_types::MultiValueVec,
};

/// Pure ABI marker for variadic (multi-value) arguments and results, i.e. `variadic<T>` in the ABI.
///
/// Zero-sized, so it can serve as `TypeAbi::Abi` for `MultiValueEncoded`, `MultiValueVec`,
/// the variadic storage mappers, etc., without dragging an allocated type along.
pub struct VariadicAbi<T>
where
    T: TypeAbi,
{
    _phantom: core::marker::PhantomData<T>,
}

impl<T, U> TypeAbiFrom<VariadicAbi<U>> for VariadicAbi<T>
where
    T: TypeAbi + TypeAbiFrom<U>,
    U: TypeAbi,
{
}

impl<T, U> TypeAbiFrom<MultiValueVec<U>> for VariadicAbi<T> where T: TypeAbi + TypeAbiFrom<U> {}

impl<T, U> TypeAbiFrom<VariadicAbi<U>> for MultiValueVec<T>
where
    T: TypeAbiFrom<U>,
    U: TypeAbi,
{
}

impl<T> TypeAbi for VariadicAbi<T>
where
    T: TypeAbi,
{
    type Abi = Self;

    fn type_name() -> TypeName {
        let mut repr = TypeName::from("variadic<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }

    fn type_name_rust() -> TypeName {
        let mut repr = TypeName::from("VariadicAbi<");
        repr.push_str(T::type_name_rust().as_str());
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
