use core::marker::PhantomData;

use crate::{AbiType, AbiTypeFrom, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};

pub struct MultiValueListAbi<T: AbiType>(PhantomData<T>);

impl<T, U> AbiTypeFrom<MultiValueListAbi<U>> for MultiValueListAbi<T>
where
    T: AbiTypeFrom<U>,
    U: AbiType,
{
}

impl<T: AbiType> AbiType for MultiValueListAbi<T> {
    fn type_name() -> TypeName {
        crate::type_name_variadic::<T>()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}

impl<T: AbiType> TypeAbiFrom<Self> for MultiValueListAbi<T> {}

impl<T: AbiType> TypeAbi for MultiValueListAbi<T> {
    type Abi = Self;
}

pub struct CountedVariadicAbi<T: AbiType>(PhantomData<T>);

impl<T: AbiType> AbiTypeFrom<Self> for CountedVariadicAbi<T> {}

impl<T: AbiType> AbiType for CountedVariadicAbi<T> {
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

impl<T: AbiType> TypeAbiFrom<Self> for CountedVariadicAbi<T> {}

impl<T: AbiType> TypeAbi for CountedVariadicAbi<T> {
    type Abi = Self;
}

pub struct BytesReadToEndAbi;

impl AbiTypeFrom<Self> for BytesReadToEndAbi {}

impl AbiType for BytesReadToEndAbi {
    fn type_name() -> TypeName {
        "bytes-read-to-end".into()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for BytesReadToEndAbi {}

impl TypeAbi for BytesReadToEndAbi {
    type Abi = Self;
}
