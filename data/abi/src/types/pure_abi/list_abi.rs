use crate::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};

pub struct ListAbi<T>
where
    T: TypeAbi,
{
    _phantom: core::marker::PhantomData<T>,
}

impl<T> TypeAbiFrom<Self> for ListAbi<T> where T: TypeAbi {}

impl<T> TypeAbi for ListAbi<T>
where
    T: TypeAbi,
{
    type Unmanaged = Self;
    type Abi = Self;

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

    fn type_name_rust() -> TypeName {
        let mut repr = TypeName::from("ListAbi<");
        repr.push_str(T::type_name_rust().as_str());
        repr.push('>');
        repr
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}
