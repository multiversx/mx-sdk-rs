use alloc::format;

use crate::{
    abi::{OutputAbis, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName},
    codec::multi_types::{IgnoreValue, OptionalValue},
};

#[cfg(feature = "alloc")]
impl<T, U> TypeAbiFrom<crate::codec::multi_types::MultiValueVec<U>>
    for crate::codec::multi_types::MultiValueVec<T>
where
    T: TypeAbiFrom<U>,
{
}

#[cfg(feature = "alloc")]
impl<T: TypeAbi> TypeAbi for crate::codec::multi_types::MultiValueVec<T> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        super::type_name_variadic::<T>()
    }

    fn type_name_rust() -> TypeName {
        format!("MultiValueVec<{}>", T::type_name_rust())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}

impl<T> TypeAbiFrom<T> for IgnoreValue {}

impl TypeAbi for IgnoreValue {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from("ignore")
    }

    fn type_name_rust() -> TypeName {
        "IgnoreValue".into()
    }

    fn is_variadic() -> bool {
        true
    }
}

impl<T, U> TypeAbiFrom<OptionalValue<T>> for OptionalValue<U> where T: TypeAbiFrom<U> {}

impl<T: TypeAbi> TypeAbi for OptionalValue<T> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        super::type_name_optional::<T>()
    }

    fn type_name_rust() -> TypeName {
        format!("OptionalValue<{}>", T::type_name_rust())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}

macro_rules! multi_arg_impls {
    ($(($mval_struct:ident $($n:tt $name:ident)+) )+) => {
        $(
            impl<$($name),+ > TypeAbiFrom<Self> for crate::codec::multi_types::$mval_struct<$($name,)+>
            where
                $($name: TypeAbi,)+
            {}

            impl<$($name),+ > TypeAbi for crate::codec::multi_types::$mval_struct<$($name,)+>
            where
                $($name: TypeAbi,)+
            {
                type Unmanaged = Self;

                fn type_name() -> TypeName {
                    let mut repr = TypeName::from("multi");
                    repr.push('<');
                    $(
                        if $n > 0 {
                            repr.push(',');
                        }
                        repr.push_str($name::type_name().as_str());
                    )+
                    repr.push('>');
                    repr
                }

                fn type_name_rust() -> TypeName {
                    let mut repr = TypeName::from(stringify!($mval_struct));
                    repr.push('<');
                    $(
                        if $n > 0 {
                            repr.push_str(", ");
                        }
                        repr.push_str($name::type_name_rust().as_str());
                    )+
                    repr.push('>');
                    repr
                }

                fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
                    $(
                        $name::provide_type_descriptions(accumulator);
                    )+
                }

                fn is_variadic() -> bool {
                    true
                }

                fn output_abis(output_names: &[&'static str]) -> OutputAbis {
                    let mut result = OutputAbis::new();
                    $(
                        if output_names.len() > $n {
                            result.append(&mut $name::output_abis(&[output_names[$n]]));

                        } else {
                            result.append(&mut $name::output_abis(&[]));
                        }

                    )+
                    result
                }
            }
        )+
    }
}

multi_arg_impls! {
    (MultiValue2  0 T0 1 T1)
    (MultiValue3  0 T0 1 T1 2 T2)
    (MultiValue4  0 T0 1 T1 2 T2 3 T3)
    (MultiValue5  0 T0 1 T1 2 T2 3 T3 4 T4)
    (MultiValue6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (MultiValue7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (MultiValue8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (MultiValue9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (MultiValue10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (MultiValue11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (MultiValue12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (MultiValue13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (MultiValue14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (MultiValue15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (MultiValue16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
