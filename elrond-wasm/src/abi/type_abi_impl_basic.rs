use super::*;
use crate::arrayvec::ArrayVec;
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

impl TypeAbi for () {
    /// No another exception from the 1-type-1-output-abi rule:
    /// the unit type produces no output.
    fn output_abis(_output_names: &[&'static str]) -> OutputAbis {
        Vec::new()
    }
}

impl<T: TypeAbi> TypeAbi for &T {
    fn type_name() -> TypeName {
        T::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T: TypeAbi> TypeAbi for Box<T> {
    fn type_name() -> TypeName {
        T::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T: TypeAbi> TypeAbi for &[T] {
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

impl<T: TypeAbi> TypeAbi for Vec<T> {
    fn type_name() -> TypeName {
        <&[T]>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T: TypeAbi, const CAP: usize> TypeAbi for ArrayVec<T, CAP> {
    fn type_name() -> TypeName {
        <&[T]>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T: TypeAbi> TypeAbi for Box<[T]> {
    fn type_name() -> TypeName {
        <&[T]>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl TypeAbi for String {
    fn type_name() -> TypeName {
        "utf-8 string".into()
    }
}

impl TypeAbi for &str {
    fn type_name() -> TypeName {
        TypeName::type_name()
    }
}

impl TypeAbi for Box<str> {
    fn type_name() -> TypeName {
        TypeName::type_name()
    }
}

macro_rules! type_abi_name_only {
    ($ty:ty, $name:expr) => {
        impl TypeAbi for $ty {
            fn type_name() -> TypeName {
                TypeName::from($name)
            }

            fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
        }
    };
}

type_abi_name_only!(u8, "u8");
type_abi_name_only!(u16, "u16");
type_abi_name_only!(u32, "u32");
type_abi_name_only!(usize, "u32");
type_abi_name_only!(u64, "u64");

type_abi_name_only!(i8, "i8");
type_abi_name_only!(i16, "i16");
type_abi_name_only!(i32, "i32");
type_abi_name_only!(isize, "i32");
type_abi_name_only!(i64, "i64");

type_abi_name_only!(core::num::NonZeroUsize, "NonZeroUsize");
type_abi_name_only!(bool, "bool");

impl<T: TypeAbi> TypeAbi for Option<T> {
    fn type_name() -> TypeName {
        let mut repr = TypeName::from("Option<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T: TypeAbi, E> TypeAbi for Result<T, E> {
    fn type_name() -> TypeName {
        T::type_name()
    }

    /// Similar to the SCResult implementation.
    fn output_abis(output_names: &[&'static str]) -> OutputAbis {
        T::output_abis(output_names)
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> TypeAbi for ($($name,)+)
            where
                $($name: TypeAbi,)+
            {
				fn type_name() -> TypeName {
					let mut repr = TypeName::from("tuple");
					repr.push_str("<");
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

impl<T: TypeAbi, const N: usize> TypeAbi for [T; N] {
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
