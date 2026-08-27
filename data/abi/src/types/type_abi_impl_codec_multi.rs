use alloc::format;

use crate::{
    AbiType, AbiTypeFrom, OutputAbis, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName,
    codec::multi_types::{IgnoreValue, OptionalValue},
};

impl<T, U> TypeAbiFrom<crate::codec::multi_types::MultiValueVec<U>>
    for crate::codec::multi_types::MultiValueVec<T>
where
    T: TypeAbiFrom<U>,
{
}

impl<T: AbiType> AbiTypeFrom<Self> for crate::codec::multi_types::MultiValueVec<T> {}

impl<T: AbiType> AbiType for crate::codec::multi_types::MultiValueVec<T> {
    fn type_name() -> TypeName {
        super::type_name_variadic::<T>()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}

impl<T: TypeAbi> TypeAbi for crate::codec::multi_types::MultiValueVec<T> {
    type Abi = crate::codec::multi_types::MultiValueVec<T::Abi>;

    fn type_name_rust() -> TypeName {
        format!("MultiValueVec<{}>", T::type_name_rust())
    }

    fn is_variadic() -> bool {
        true
    }
}

impl TypeAbiFrom<IgnoreValue> for IgnoreValue {}
impl AbiTypeFrom<Self> for IgnoreValue {}

impl AbiType for IgnoreValue {
    fn type_name() -> TypeName {
        TypeName::from("ignore")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}

    fn is_variadic() -> bool {
        true
    }
}

impl TypeAbi for IgnoreValue {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "IgnoreValue".into()
    }

    fn is_variadic() -> bool {
        true
    }
}

impl<T, U> TypeAbiFrom<OptionalValue<U>> for OptionalValue<T> where T: TypeAbiFrom<U> {}

// IgnoreValue -> OptionalValue::None
// It makes it easier with
impl<T> TypeAbiFrom<IgnoreValue> for OptionalValue<T> {}
impl<T: AbiType> AbiTypeFrom<Self> for OptionalValue<T> {}

impl<T: AbiType> AbiType for OptionalValue<T> {
    fn type_name() -> TypeName {
        super::type_name_optional::<T>()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}

impl<T: TypeAbi> TypeAbi for OptionalValue<T> {
    type Abi = OptionalValue<T::Abi>;

    fn type_name_rust() -> TypeName {
        format!("OptionalValue<{}>", T::type_name_rust())
    }

    fn is_variadic() -> bool {
        true
    }
}

macro_rules! multi_arg_impls {
    ($(($mval_struct:ident $($n:tt $t:ident $u:ident)+) )+) => {
        $(
            impl<$($t, $u),+> TypeAbiFrom<crate::codec::multi_types::$mval_struct<$($u,)+>> for crate::codec::multi_types::$mval_struct<$($t,)+>
            where
                $($t: TypeAbiFrom<$u>,)+
            {}

            impl<$($t),+> AbiTypeFrom<Self> for crate::codec::multi_types::$mval_struct<$($t,)+>
            where
                $($t: AbiType,)+
            {}

            impl<$($t),+> AbiType for crate::codec::multi_types::$mval_struct<$($t,)+>
            where
                $($t: AbiType,)+
            {
                fn type_name() -> TypeName {
                    let mut repr = TypeName::from("multi");
                    repr.push('<');
                    $(
                        if $n > 0 {
                            repr.push(',');
                        }
                        repr.push_str($t::type_name().as_str());
                    )+
                    repr.push('>');
                    repr
                }

                fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
                    $(
                        $t::provide_type_descriptions(accumulator);
                    )+
                }

                fn is_variadic() -> bool {
                    true
                }
            }

            impl<$($t),+> TypeAbi for crate::codec::multi_types::$mval_struct<$($t,)+>
            where
                $($t: TypeAbi,)+
            {
                type Abi = crate::codec::multi_types::$mval_struct<$($t::Abi,)+>;

                fn type_name_rust() -> TypeName {
                    let mut repr = TypeName::from(stringify!($mval_struct));
                    repr.push('<');
                    $(
                        if $n > 0 {
                            repr.push_str(", ");
                        }
                        repr.push_str($t::type_name_rust().as_str());
                    )+
                    repr.push('>');
                    repr
                }

                fn is_variadic() -> bool {
                    true
                }

                fn output_abis(output_names: &[&'static str]) -> OutputAbis {
                    let mut result = OutputAbis::new();
                    $(
                        if output_names.len() > $n {
                            result.append(&mut $t::output_abis(&[output_names[$n]]));

                        } else {
                            result.append(&mut $t::output_abis(&[]));
                        }

                    )+
                    result
                }
            }


        )+
    }
}

multi_arg_impls! {
    (MultiValue2 0 T0 U0 1 T1 U1)
    (MultiValue3 0 T0 U0 1 T1 U1 2 T2 U2)
    (MultiValue4 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3)
    (MultiValue5 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4)
    (MultiValue6 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4 5 T5 U5)
    (MultiValue7 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4 5 T5 U5 6 T6 U6)
    (MultiValue8 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4 5 T5 U5 6 T6 U6 7 T7 U7)
    (MultiValue9 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4 5 T5 U5 6 T6 U6 7 T7 U7 8 T8 U8)
    (MultiValue10 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4 5 T5 U5 6 T6 U6 7 T7 U7 8 T8 U8 9 T9 U9)
    (MultiValue11 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4 5 T5 U5 6 T6 U6 7 T7 U7 8 T8 U8 9 T9 U9 10 T10 U10)
    (MultiValue12 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4 5 T5 U5 6 T6 U6 7 T7 U7 8 T8 U8 9 T9 U9 10 T10 U10 11 T11 U11)
    (MultiValue13 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4 5 T5 U5 6 T6 U6 7 T7 U7 8 T8 U8 9 T9 U9 10 T10 U10 11 T11 U11 12 T12 U12)
    (MultiValue14 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4 5 T5 U5 6 T6 U6 7 T7 U7 8 T8 U8 9 T9 U9 10 T10 U10 11 T11 U11 12 T12 U12 13 T13 U13)
    (MultiValue15 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4 5 T5 U5 6 T6 U6 7 T7 U7 8 T8 U8 9 T9 U9 10 T10 U10 11 T11 U11 12 T12 U12 13 T13 U13 14 T14 U14)
    (MultiValue16 0 T0 U0 1 T1 U1 2 T2 U2 3 T3 U3 4 T4 U4 5 T5 U5 6 T6 U6 7 T7 U7 8 T8 U8 9 T9 U9 10 T10 U10 11 T11 U11 12 T12 U12 13 T13 U13 14 T14 U14 15 T15 U15)
}
