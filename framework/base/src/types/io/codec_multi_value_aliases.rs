use crate::{
    codec,
    codec::multi_types::{IgnoreValue, OptionalValue},
};

/// Structure that allows taking a variable number of arguments
/// or returning a variable number of results in a smart contract endpoint.
#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueVec`"
)]
#[cfg(feature = "alloc")]
pub type MultiArgVec<T> = codec::multi_types::MultiValueVec<T>;

/// Used for taking a variable number of arguments in an endpoint,
/// it is synonymous with `MultiResultVec`/`MultiArgVec`.
#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueVec`"
)]
#[cfg(feature = "alloc")]
pub type VarArgs<T> = codec::multi_types::MultiValueVec<T>;

/// Used for returning a variable number of results from an endpoint,
/// it is synonymous with `MultiResult`.
#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `MultiValueVec`"
)]
#[cfg(feature = "alloc")]
pub type MultiResultVec<T> = codec::multi_types::MultiValueVec<T>;

/// Structure that allows taking a variable number of arguments,
/// but does nothing with them, not even deserialization.
#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `IgnoreValue`"
)]
pub type IgnoreVarArgs = IgnoreValue;

/// A smart contract argument or result that can be missing.
///
/// If arguments stop before this argument, None will be returned.
/// As an endpoint result, the contract decides if it produces it or not.
///
/// As a principle, optional arguments or results should come last,
/// otherwise there is ambiguity as to how to interpret what comes after.
#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `OptionalValue`"
)]
pub type OptionalArg<T> = OptionalValue<T>;

/// It is just an alias for `OptionalArg`.
/// In general we use `OptionalArg` for arguments and `OptionalResult` for results,
/// but it is the same implementation for both.
#[deprecated(
    since = "0.29.0",
    note = "Alias kept for backwards compatibility. Replace with `OptionalValue`"
)]
pub type OptionalResult<T> = OptionalArg<T>;

macro_rules! multi_arg_impls {
    ($(($mval_struct:ident $marg_struct:ident $mres_struct:ident $($n:tt $name:ident)+) )+) => {
        $(
            #[deprecated(
                since = "0.29.0",
                note = "Alias kept for backwards compatibility. Replace with `MultiValue*`"
            )]
            pub type $marg_struct<$($name,)+> = codec::multi_types::$mval_struct<$($name,)+>;

            #[deprecated(
                since = "0.29.0",
                note = "Alias kept for backwards compatibility. Replace with `MultiValue*`"
            )]
            pub type $mres_struct<$($name,)+> = codec::multi_types::$mval_struct<$($name,)+>;
        )+
    }
}

multi_arg_impls! {
    (MultiValue2  MultiArg2  MultiResult2  0 T0 1 T1)
    (MultiValue3  MultiArg3  MultiResult3  0 T0 1 T1 2 T2)
    (MultiValue4  MultiArg4  MultiResult4  0 T0 1 T1 2 T2 3 T3)
    (MultiValue5  MultiArg5  MultiResult5  0 T0 1 T1 2 T2 3 T3 4 T4)
    (MultiValue6  MultiArg6  MultiResult6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (MultiValue7  MultiArg7  MultiResult7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (MultiValue8  MultiArg8  MultiResult8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (MultiValue9  MultiArg9  MultiResult9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (MultiValue10 MultiArg10 MultiResult10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (MultiValue11 MultiArg11 MultiResult11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (MultiValue12 MultiArg12 MultiResult12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (MultiValue13 MultiArg13 MultiResult13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (MultiValue14 MultiArg14 MultiResult14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (MultiValue15 MultiArg15 MultiResult15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (MultiValue16 MultiArg16 MultiResult16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
