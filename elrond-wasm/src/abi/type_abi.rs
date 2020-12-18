use super::*;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct TypeDescription {
	pub docs: &'static [&'static str],
	pub name: String,
	pub contents: TypeContents,
}

#[derive(Clone, Debug)]
pub enum TypeContents {
	NotSpecified,
	Enum(Vec<EnumVariantDescription>),
	Struct,
}

impl TypeContents {
	pub fn is_specified(&self) -> bool {
		match *self {
			TypeContents::NotSpecified => false,
			_ => true,
		}
	}
}

#[derive(Clone, Debug)]
pub struct EnumVariantDescription {
	pub docs: &'static [&'static str],
	pub name: &'static str,
}

pub trait TypeAbi {
	fn type_name() -> String {
		core::any::type_name::<Self>().into()
	}

	fn output_abis() -> Vec<OutputAbi> {
		let mut result = Vec::with_capacity(1);
		result.push(OutputAbi {
			type_description: Self::type_description(),
			variable_num: false,
		});
		result
	}

	fn type_description() -> TypeDescription {
		TypeDescription {
			docs: &[],
			name: Self::type_name(),
			contents: TypeContents::NotSpecified,
		}
	}
}

impl TypeAbi for () {
	fn output_abis() -> Vec<OutputAbi> {
		Vec::new()
	}
}

impl<T: TypeAbi> TypeAbi for &T {
	fn type_name() -> String {
		T::type_name()
	}

	fn output_abis() -> Vec<OutputAbi> {
		T::output_abis()
	}

	fn type_description() -> TypeDescription {
		T::type_description()
	}
}

impl<T: TypeAbi> TypeAbi for Box<T> {
	fn type_name() -> String {
		T::type_name()
	}

	fn output_abis() -> Vec<OutputAbi> {
		T::output_abis()
	}

	fn type_description() -> TypeDescription {
		T::type_description()
	}
}

impl<T: TypeAbi> TypeAbi for &[T] {
	fn type_name() -> String {
		let mut repr = String::from("List<");
		repr.push_str(T::type_name().as_str());
		repr.push('>');
		repr
	}
}

impl<T: TypeAbi> TypeAbi for Vec<T> {
	fn type_name() -> String {
		<&[T]>::type_name()
	}
}

impl<T: TypeAbi> TypeAbi for Box<[T]> {
	fn type_name() -> String {
		<&[T]>::type_name()
	}
}

macro_rules! type_abi_name_only {
	($ty:ty, $name:expr) => {
		impl TypeAbi for $ty {
			fn type_name() -> String {
				String::from($name)
			}
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
	fn type_name() -> String {
		let mut repr = String::from("Option<");
		repr.push_str(T::type_name().as_str());
		repr.push('>');
		repr
	}
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> TypeAbi for ($($name,)+)
            where
                $($name: TypeAbi,)+
            {
				fn type_name() -> String {
					let mut repr = String::from("(");
					$(
						if $n > 0 {
							repr.push(',');
						}
						repr.push_str($name::type_name().as_str());
                    )+
					repr.push(')');
					repr
				}
            }
        )+
    }
}

tuple_impls! {
	1 => (0 T0)
	2 => (0 T0 1 T1)
	3 => (0 T0 1 T1 2 T2)
	4 => (0 T0 1 T1 2 T2 3 T3)
	5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
	6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
	7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
	8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
	9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
	10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
	11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
	12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
	13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
	14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
	15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
	16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

macro_rules! array_impls {
    ($($n: tt,)+) => {
        $(
            impl<T: TypeAbi> TypeAbi for [T; $n] {
				fn type_name() -> String {
					let mut repr = String::from("[");
					repr.push_str(T::type_name().as_str());
					repr.push_str("; ");
					repr.push_str(stringify!($n));
					repr.push(']');
					repr
				}
			}
        )+
    }
}

#[rustfmt::skip]
array_impls!(
	1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
	17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
	32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
	52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
	72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
	92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
	109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124,
	125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140,
	141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156,
	157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172,
	173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188,
	189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204,
	205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
	221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236,
	237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252,
	253, 254, 255, 256, 384, 512, 768, 1024, 2048, 4096, 8192, 16384, 32768,
);
