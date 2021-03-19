use super::sc_error::SCError;
use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::{EndpointFinishApi, ErrorApi};
use crate::EndpointResult;
use crate::*;

/// Default way to optionally return an error from a smart contract endpoint.
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub enum SCResult<T> {
	Ok(T),
	Err(SCError),
}

impl<T> SCResult<T> {
	pub fn is_ok(&self) -> bool {
		matches!(self, SCResult::Ok(_))
	}

	pub fn is_err(&self) -> bool {
		!self.is_ok()
	}

	#[inline]
	pub fn ok(self) -> Option<T> {
		if let SCResult::Ok(t) = self {
			Some(t)
		} else {
			None
		}
	}

	#[inline]
	pub fn err(self) -> Option<SCError> {
		if let SCResult::Err(e) = self {
			Some(e)
		} else {
			None
		}
	}

	/// Used to convert from a regular Rust result.
	/// Any error type is accepted as long as it can be converted to a SCError
	/// (`Vec<u8>`, `&[u8]`, `BoxedBytes`, `String`, `&str` are covered).
	pub fn from_result<E>(r: core::result::Result<T, E>) -> Self
	where
		E: Into<SCError>,
	{
		match r {
			Ok(t) => SCResult::Ok(t),
			Err(e) => SCResult::Err(e.into()),
		}
	}
}

impl<FA, T> EndpointResult<FA> for SCResult<T>
where
	FA: EndpointFinishApi + ErrorApi + Clone + 'static,
	T: EndpointResult<FA>,
{
	#[inline]
	fn finish(&self, api: FA) {
		match self {
			SCResult::Ok(t) => {
				t.finish(api);
			},
			SCResult::Err(e) => {
				api.signal_error(e.as_bytes());
			},
		}
	}
}

impl<T: TypeAbi> TypeAbi for SCResult<T> {
	fn type_name() -> String {
		T::type_name()
	}

	/// Gives `SCResult<()>` the possibility to produce 0 output ABIs,
	/// just like `()`.
	/// It is also possible to have `SCResult<MultiResultX<...>>`,
	/// so this gives the MultiResult to dissolve into its multiple output ABIs.
	fn output_abis(output_names: &[&'static str]) -> Vec<OutputAbi> {
		T::output_abis(output_names)
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
		T::provide_type_descriptions(accumulator);
	}
}

impl<T> SCResult<T> {
	pub fn unwrap(self) -> T {
		match self {
			SCResult::Ok(t) => t,
			SCResult::Err(_) => panic!("called `SCResult::unwrap()`"),
		}
	}
}
