use crate::abi::{TypeAbi, TypeDescriptionContainer};
use alloc::string::String;
use elrond_codec::multi_types::MultiValueVec;

/// Structure that allows taking a variable number of arguments
/// or returning a variable number of results in a smart contract endpoint.
#[deprecated(
    since = "0.28.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueVec`"
)]
pub type MultiArgVec<T> = MultiValueVec<T>;

/// Used for taking a variable number of arguments in an endpoint,
/// it is synonymous with `MultiResultVec`/`MultiArgVec`.
#[deprecated(
    since = "0.28.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueVec`"
)]
pub type VarArgs<T> = MultiArgVec<T>;

/// Used for returning a variable number of results from an endpoint,
/// it is synonymous with `MultiResult`.
#[deprecated(
    since = "0.28.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueVec`"
)]
pub type MultiResultVec<T> = VarArgs<T>;

impl<T: TypeAbi> TypeAbi for MultiArgVec<T> {
    fn type_name() -> String {
        let mut repr = String::from("variadic<");
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
