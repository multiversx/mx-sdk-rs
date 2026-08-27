use crate::{AbiType, AbiTypeFrom, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};

pub struct ListAbi<T>
where
    T: AbiType,
{
    _phantom: core::marker::PhantomData<T>,
}

pub type BytesAbi = ListAbi<u8>;

impl<T> AbiTypeFrom<Self> for ListAbi<T> where T: AbiType {}

impl<T, U> AbiTypeFrom<alloc::vec::Vec<U>> for ListAbi<T>
where
    T: AbiTypeFrom<U>,
    U: AbiType,
{
}

impl<T, U> AbiTypeFrom<alloc::boxed::Box<[U]>> for ListAbi<T>
where
    T: AbiTypeFrom<U>,
    U: AbiType,
{
}

impl<T, U, const N: usize> AbiTypeFrom<[U; N]> for ListAbi<T>
where
    T: AbiTypeFrom<U>,
    U: AbiType,
{
}

impl AbiTypeFrom<alloc::string::String> for BytesAbi {}
impl AbiTypeFrom<BytesAbi> for alloc::string::String {}

impl<T> AbiType for ListAbi<T>
where
    T: AbiType,
{
    fn type_name() -> TypeName {
        let t_name = T::type_name();
        if t_name == "u8" {
            return "bytes".into();
        }
        let mut repr = TypeName::from("List<");
        repr.push_str(t_name.as_str());
        repr.push('>');
        repr
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T> TypeAbiFrom<Self> for ListAbi<T> where T: AbiType {}

impl<T> TypeAbi for ListAbi<T>
where
    T: AbiType,
{
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        let mut repr = TypeName::from("ListAbi<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }
}
