use crate::abi::{TypeAbi, TypeDescriptionContainer};
use crate::io::{ArgId, DynArg, DynArgInput};
use alloc::string::String;
use elrond_codec::TopDecodeInput;

/// A smart contract argument that can be provided or not.
/// If arguments stop before this argument, None will be returned.
pub enum OptionalArg<T> {
	Some(T),
	None,
}

impl<T> From<Option<T>> for OptionalArg<T> {
	fn from(v: Option<T>) -> Self {
		match v {
			Some(arg) => OptionalArg::Some(arg),
			None => OptionalArg::None,
		}
	}
}

impl<T> OptionalArg<T> {
	pub fn into_option(self) -> Option<T> {
		match self {
			OptionalArg::Some(arg) => Some(arg),
			OptionalArg::None => None,
		}
	}
}

impl<T> DynArg for OptionalArg<T>
where
	T: DynArg,
{
	fn dyn_load<I, D>(loader: &mut D, arg_id: ArgId) -> Self
	where
		I: TopDecodeInput,
		D: DynArgInput<I>,
	{
		if loader.has_next() {
			OptionalArg::Some(T::dyn_load(loader, arg_id))
		} else {
			OptionalArg::None
		}
	}
}

impl<T: TypeAbi> TypeAbi for OptionalArg<T> {
	fn type_name() -> String {
		let mut repr = String::from("OptionalArg<");
		repr.push_str(T::type_name().as_str());
		repr.push('>');
		repr
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
		T::provide_type_descriptions(accumulator);
	}

	fn is_multi_arg_or_result() -> bool {
		true
	}
}
