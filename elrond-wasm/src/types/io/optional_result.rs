use crate::abi::{TypeAbi, TypeDescriptionContainer};
use crate::{api::EndpointFinishApi, EndpointResult};
use alloc::string::String;

#[must_use]
pub enum OptionalResult<T> {
	Some(T),
	None,
}

impl<T> From<Option<T>> for OptionalResult<T> {
	fn from(v: Option<T>) -> Self {
		match v {
			Some(result) => OptionalResult::Some(result),
			None => OptionalResult::None,
		}
	}
}

impl<FA, T> EndpointResult<FA> for OptionalResult<T>
where
	FA: EndpointFinishApi + Clone + 'static,
	T: EndpointResult<FA>,
{
	#[inline]
	fn finish(&self, api: FA) {
		if let OptionalResult::Some(t) = self {
			t.finish(api);
		}
	}
}

impl<T: TypeAbi> TypeAbi for OptionalResult<T> {
	fn type_name() -> String {
		let mut repr = String::from("OptionalResult<");
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
