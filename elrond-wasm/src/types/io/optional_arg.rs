use crate::abi::{TypeAbi, TypeDescriptionContainer};
use alloc::string::String;
use elrond_codec::multi_types::OptionalValue;

/// A smart contract argument or result that can be missing.
///
/// If arguments stop before this argument, None will be returned.
/// As an endpoint result, the contract decides if it produces it or not.
///
/// As a principle, optional arguments or results should come last,
/// otherwise there is ambiguity as to how to interpret what comes after.
#[deprecated(
    since = "0.28.0",
    note = "Alias kept for backwards compatibility. Replace with `OptionalValue`"
)]
pub type OptionalArg<T> = OptionalValue<T>;

/// It is just an alias for `OptionalArg`.
/// In general we use `OptionalArg` for arguments and `OptionalResult` for results,
/// but it is the same implementation for both.
#[deprecated(
    since = "0.28.0",
    note = "Alias kept for backwards compatibility. Replace with `OptionalValue`"
)]
pub type OptionalResult<T> = OptionalArg<T>;

impl<T: TypeAbi> TypeAbi for OptionalValue<T> {
    fn type_name() -> String {
        let mut repr = String::from("optional<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}
