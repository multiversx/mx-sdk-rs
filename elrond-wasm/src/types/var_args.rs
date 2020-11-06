use crate::io::{ArgId, DynArg, DynArgInput, DynArgMulti};
use alloc::vec::Vec;
use elrond_codec::TopDecodeInput;

/// Structure that allows taking a variable number of arguments in a smart contract function.
pub struct VarArgs<T>(pub Vec<T>);

impl<T> From<Vec<T>> for VarArgs<T> {
	fn from(v: Vec<T>) -> Self {
		VarArgs(v)
	}
}

impl<T> VarArgs<T> {
	#[inline]
	pub fn new() -> Self {
		VarArgs(Vec::new())
	}
}

impl<T> Default for VarArgs<T> {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl<T> VarArgs<T> {
	#[inline]
	pub fn into_vec(self) -> Vec<T> {
		self.0
	}

	#[inline]
	pub fn as_slice(&self) -> &[T] {
		self.0.as_slice()
	}

	#[inline]
	pub fn push(&mut self, value: T) {
		self.0.push(value);
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	#[inline]
	pub fn iter(&self) -> core::slice::Iter<'_, T> {
		self.0.iter()
	}
}

impl<I, D, T> DynArg<I, D> for VarArgs<T>
where
	I: TopDecodeInput,
	D: DynArgInput<I>,
	T: DynArg<I, D>,
{
	// #[inline(never)]
	fn dyn_load(loader: &mut D, arg_id: ArgId) -> Self {
		let mut result_vec: Vec<T> = Vec::new();
		while loader.has_next() {
			result_vec.push(T::dyn_load(loader, arg_id));
		}
		VarArgs(result_vec)
	}
}

impl<I, D, T> DynArgMulti<I, D> for VarArgs<T>
where
	I: TopDecodeInput,
	D: DynArgInput<I>,
	T: DynArg<I, D>,
{
	fn dyn_load_multi(loader: &mut D, arg_id: ArgId, num: usize) -> Self {
		let mut result_vec: Vec<T> = Vec::new();
		let mut i = 0usize;
		while loader.has_next() && i < num {
			result_vec.push(T::dyn_load(loader, arg_id));
			i += 1;
		}
		if i < num {
			loader.signal_arg_wrong_number();
		}
		VarArgs(result_vec)
	}
}
