use super::*;
use crate::{codec::arrayvec::ArrayVec, contract_abi::OutputAbis};
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};

impl TypeAbiFrom<()> for () {}
impl AbiTypeFrom<Self> for () {}

impl AbiType for () {
    fn type_name() -> TypeName {
        TypeName::from("()")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for () {
    type Abi = Self;

    /// No another exception from the 1-type-1-output-abi rule:
    /// the unit type produces no output.
    fn output_abis(_output_names: &[&'static str]) -> OutputAbis {
        Vec::new()
    }
}

impl<T, U> TypeAbiFrom<&U> for &T where T: TypeAbiFrom<U> {}

impl<T: TypeAbi> TypeAbi for &T {
    type Abi = T::Abi;

    fn type_name_rust() -> TypeName {
        T::type_name_rust()
    }
}

impl<T, U> TypeAbiFrom<Box<U>> for Box<T> where T: TypeAbiFrom<U> {}
impl<T, U> AbiTypeFrom<Box<U>> for Box<T>
where
    T: AbiTypeFrom<U>,
    U: AbiType,
{
}

impl<T: AbiType> AbiType for Box<T> {
    fn type_name() -> TypeName {
        T::type_name()
    }

    fn type_name_specific() -> Option<TypeName> {
        T::type_name_specific()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T: TypeAbi> TypeAbi for Box<T> {
    type Abi = Box<T::Abi>;

    fn type_name_rust() -> TypeName {
        format!("Box<{}>", T::type_name_rust())
    }
}

impl<T, U> TypeAbiFrom<&[T]> for &[U] where T: TypeAbiFrom<U> {}

impl<T: AbiType> AbiType for Vec<T> {
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

impl<T: TypeAbi> TypeAbi for &[T] {
    type Abi = Vec<T::Abi>;

    fn type_name_rust() -> TypeName {
        // we need to convert to an owned type
        format!("Box<[{}]>", T::type_name_rust())
    }
}

impl<T, U> TypeAbiFrom<Vec<U>> for Vec<T> where T: TypeAbiFrom<U> {}
impl<T, U> AbiTypeFrom<Vec<U>> for Vec<T>
where
    T: AbiTypeFrom<U>,
    U: AbiType,
{
}

impl<T, U> AbiTypeFrom<ListAbi<U>> for Vec<T>
where
    T: AbiTypeFrom<U>,
    U: AbiType,
{
}

impl<T: TypeAbi> TypeAbi for Vec<T> {
    type Abi = Vec<T::Abi>;

    fn type_name_rust() -> TypeName {
        format!("Vec<{}>", T::type_name_rust())
    }
}

impl<T: TypeAbi, const CAP: usize> TypeAbiFrom<ArrayVec<T, CAP>> for ArrayVec<T, CAP> {}
impl<T: AbiType, const CAP: usize> AbiTypeFrom<Self> for ArrayVec<T, CAP> {}

impl<T: AbiType, const CAP: usize> AbiType for ArrayVec<T, CAP> {
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

impl<T: TypeAbi, const CAP: usize> TypeAbi for ArrayVec<T, CAP> {
    type Abi = ArrayVec<T::Abi, CAP>;

    fn type_name_rust() -> TypeName {
        format!("ArrayVec<{}, {}usize>", T::type_name_rust(), CAP)
    }
}

impl<T> TypeAbiFrom<Box<[T]>> for Box<[T]> {}
impl<T: AbiType> AbiTypeFrom<Self> for Box<[T]> {}

impl<T: AbiType> AbiType for Box<[T]> {
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

impl<T: TypeAbi> TypeAbi for Box<[T]> {
    type Abi = Box<[T::Abi]>;

    fn type_name_rust() -> TypeName {
        format!("Box<[{}]>", T::type_name_rust())
    }
}

impl TypeAbiFrom<String> for String {}
impl TypeAbiFrom<&String> for String {}
impl TypeAbiFrom<&str> for String {}
impl TypeAbiFrom<Box<str>> for String {}
impl AbiTypeFrom<Self> for String {}
impl AbiTypeFrom<&'static str> for String {}
impl AbiTypeFrom<Box<str>> for String {}

impl AbiType for String {
    fn type_name() -> TypeName {
        "utf-8 string".into()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for String {
    type Abi = Self;
}

impl TypeAbiFrom<&'static str> for &'static str {}
impl AbiTypeFrom<Self> for &'static str {}

impl AbiType for &'static str {
    fn type_name() -> TypeName {
        <String as AbiType>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for &'static str {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "&'static str".into()
    }
}

impl TypeAbiFrom<Box<str>> for Box<str> {}
impl TypeAbiFrom<&str> for Box<str> {}
impl TypeAbiFrom<String> for Box<str> {}
impl AbiTypeFrom<Self> for Box<str> {}
impl AbiTypeFrom<String> for Box<str> {}

impl AbiType for Box<str> {
    fn type_name() -> TypeName {
        <String as AbiType>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for Box<str> {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "Box<str>".into()
    }
}

macro_rules! type_abi_name_only {
    ($ty:ty, $name:expr) => {
        impl TypeAbiFrom<$ty> for $ty {}
        impl TypeAbiFrom<&$ty> for $ty {}
        impl AbiTypeFrom<$ty> for $ty {}

        impl AbiType for $ty {
            fn type_name() -> TypeName {
                TypeName::from($name)
            }

            fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
        }

        impl TypeAbi for $ty {
            type Abi = Self;
        }
    };
}

type_abi_name_only!(u8, "u8");
type_abi_name_only!(u16, "u16");
type_abi_name_only!(u32, "u32");
type_abi_name_only!(usize, "u32");
type_abi_name_only!(u64, "u64");
type_abi_name_only!(u128, "u128");

type_abi_name_only!(i8, "i8");
type_abi_name_only!(i16, "i16");
type_abi_name_only!(i32, "i32");
type_abi_name_only!(isize, "i32");
type_abi_name_only!(i64, "i64");

type_abi_name_only!(core::num::NonZeroUsize, "NonZeroUsize");
type_abi_name_only!(bool, "bool");
type_abi_name_only!(f64, "f64");

// Unsigned integer types: the contract can return a smaller capacity result and and we can interpret it as a larger capacity type.

impl TypeAbiFrom<u64> for u128 {}
impl TypeAbiFrom<usize> for u128 {}
impl TypeAbiFrom<u32> for u128 {}
impl TypeAbiFrom<u16> for u128 {}
impl TypeAbiFrom<u8> for u128 {}
impl AbiTypeFrom<u64> for u128 {}
impl AbiTypeFrom<usize> for u128 {}
impl AbiTypeFrom<u32> for u128 {}
impl AbiTypeFrom<u16> for u128 {}
impl AbiTypeFrom<u8> for u128 {}

impl TypeAbiFrom<usize> for u64 {}
impl TypeAbiFrom<u32> for u64 {}
impl TypeAbiFrom<u16> for u64 {}
impl TypeAbiFrom<u8> for u64 {}
impl AbiTypeFrom<usize> for u64 {}
impl AbiTypeFrom<u32> for u64 {}
impl AbiTypeFrom<u16> for u64 {}
impl AbiTypeFrom<u8> for u64 {}

impl TypeAbiFrom<usize> for u32 {}
impl TypeAbiFrom<u16> for u32 {}
impl TypeAbiFrom<u8> for u32 {}
impl AbiTypeFrom<usize> for u32 {}
impl AbiTypeFrom<u16> for u32 {}
impl AbiTypeFrom<u8> for u32 {}

impl TypeAbiFrom<u32> for usize {}
impl TypeAbiFrom<u16> for usize {}
impl TypeAbiFrom<u8> for usize {}
impl AbiTypeFrom<u32> for usize {}
impl AbiTypeFrom<u16> for usize {}
impl AbiTypeFrom<u8> for usize {}

impl TypeAbiFrom<u8> for u16 {}
impl AbiTypeFrom<u8> for u16 {}

// Signed, the same.

impl TypeAbiFrom<isize> for i64 {}
impl TypeAbiFrom<i32> for i64 {}
impl TypeAbiFrom<i16> for i64 {}
impl TypeAbiFrom<i8> for i64 {}
impl AbiTypeFrom<isize> for i64 {}
impl AbiTypeFrom<i32> for i64 {}
impl AbiTypeFrom<i16> for i64 {}
impl AbiTypeFrom<i8> for i64 {}

impl TypeAbiFrom<isize> for i32 {}
impl TypeAbiFrom<i16> for i32 {}
impl TypeAbiFrom<i8> for i32 {}
impl AbiTypeFrom<isize> for i32 {}
impl AbiTypeFrom<i16> for i32 {}
impl AbiTypeFrom<i8> for i32 {}

impl TypeAbiFrom<i32> for isize {}
impl TypeAbiFrom<i16> for isize {}
impl TypeAbiFrom<i8> for isize {}
impl AbiTypeFrom<i32> for isize {}
impl AbiTypeFrom<i16> for isize {}
impl AbiTypeFrom<i8> for isize {}

impl TypeAbiFrom<i8> for i16 {}
impl AbiTypeFrom<i8> for i16 {}

impl<T, U> TypeAbiFrom<Option<U>> for Option<T> where T: TypeAbiFrom<U> {}
impl<T, U> AbiTypeFrom<Option<U>> for Option<T>
where
    T: AbiTypeFrom<U>,
    U: AbiType,
{
}

impl<T: AbiType> AbiType for Option<T> {
    fn type_name() -> TypeName {
        format!("Option<{}>", T::type_name())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T> TypeAbi for Option<T>
where
    T: TypeAbi,
{
    type Abi = Option<T::Abi>;

    fn type_name_rust() -> TypeName {
        format!("Option<{}>", T::type_name_rust())
    }
}

impl<T: TypeAbi, E> TypeAbiFrom<Self> for Result<T, E> {}
impl<T: AbiType, E> AbiTypeFrom<Self> for Result<T, E> {}

impl<T: AbiType, E> AbiType for Result<T, E> {
    fn type_name() -> TypeName {
        T::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T: TypeAbi, E> TypeAbi for Result<T, E> {
    type Abi = Result<T::Abi, E>;

    fn type_name_rust() -> TypeName {
        format!(
            "Result<{}, {}>",
            T::type_name_rust(),
            core::any::type_name::<E>()
        )
    }

    /// Similar to the SCResult implementation.
    fn output_abis(output_names: &[&'static str]) -> OutputAbis {
        T::output_abis(output_names)
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> TypeAbiFrom<Self> for ($($name,)+)
            where
                $($name: TypeAbi,)+
            {}

            impl<$($name),+> AbiTypeFrom<Self> for ($($name,)+)
            where
                $($name: AbiType,)+
            {}

            impl<$($name),+> AbiType for ($($name,)+)
            where
                $($name: AbiType,)+
            {
                fn type_name() -> TypeName {
                    let mut repr = TypeName::from("tuple<");
                    $(
                        if $n > 0 {
                            repr.push(',');
                        }
                        repr.push_str($name::type_name().as_str());
                    )+
                    repr.push('>');
                    repr
                }

                fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
                    $(
                        $name::provide_type_descriptions(accumulator);
                    )+
                }
            }

            impl<$($name),+> TypeAbi for ($($name,)+)
            where
                $($name: TypeAbi,)+
            {
                type Abi = ($($name::Abi,)+);

                fn type_name_rust() -> TypeName {
                    let mut repr = TypeName::from("(");
                    $(
                        if $n > 0 {
                            repr.push_str(", ");
                        }
                        repr.push_str($name::type_name_rust().as_str());
                    )+
                    repr.push(')');
                    repr
                }
            }


        )+
    }
}

tuple_impls! {
    1  => (0 T0)
    2  => (0 T0 1 T1)
    3  => (0 T0 1 T1 2 T2)
    4  => (0 T0 1 T1 2 T2 3 T3)
    5  => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

impl<T, U, const N: usize> TypeAbiFrom<[U; N]> for [T; N] where T: TypeAbiFrom<U> {}
impl<T, U, const N: usize> AbiTypeFrom<[U; N]> for [T; N]
where
    T: AbiTypeFrom<U>,
    U: AbiType,
{
}

impl<T: AbiType, const N: usize> AbiType for [T; N] {
    fn type_name() -> TypeName {
        let mut repr = TypeName::from("array");
        repr.push_str(N.to_string().as_str());
        repr.push('<');
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T: TypeAbi, const N: usize> TypeAbi for [T; N] {
    type Abi = [T::Abi; N];

    fn type_name_rust() -> TypeName {
        let mut repr = TypeName::from("[");
        repr.push_str(T::type_name_rust().as_str());
        repr.push_str("; ");
        repr.push_str(N.to_string().as_str());
        repr.push(']');
        repr
    }
}
