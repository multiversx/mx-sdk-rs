use crate::io::{ArgId, DynArg, DynArgInput};
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

impl<I, D, T> DynArg<I, D> for OptionalArg<T>
where
	I: TopDecodeInput,
	D: DynArgInput<I>,
	T: DynArg<I, D>,
{
	fn dyn_load(loader: &mut D, arg_id: ArgId) -> Self {
		if loader.has_next() {
			OptionalArg::Some(T::dyn_load(loader, arg_id))
		} else {
			OptionalArg::None
		}
	}
}
