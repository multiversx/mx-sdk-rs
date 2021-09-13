use crate::{
    abi::{TypeAbi, TypeDescriptionContainer},
    api::{EndpointFinishApi, ManagedTypeApi},
    io::{ArgId, ContractCallArg, DynArg, DynArgInput, DynArgOutput},
    EndpointResult,
};
use alloc::string::String;

/// A smart contract argument or result that can be missing.
///
/// If arguments stop before this argument, None will be returned.
/// As an endpoint result, the contract decides if it produces it or not.
///
/// As a principle, optional arguments or results should come last,
/// otherwise there is ambiguity as to how to interpret what comes after.
#[must_use]
#[derive(Clone)]
pub enum OptionalArg<T> {
    Some(T),
    None,
}

/// It is just an alias for `OptionalArg`.
/// In general we use `OptionalArg` for arguments and `OptionalResult` for results,
/// but it is the same implementation for both.
pub type OptionalResult<T> = OptionalArg<T>;

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
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        if loader.has_next() {
            OptionalArg::Some(T::dyn_load(loader, arg_id))
        } else {
            OptionalArg::None
        }
    }
}

impl<T> EndpointResult for OptionalArg<T>
where
    T: EndpointResult,
{
    type DecodeAs = OptionalArg<T::DecodeAs>;

    #[inline]
    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        if let OptionalResult::Some(t) = self {
            t.finish(api);
        }
    }
}

impl<T> ContractCallArg for &OptionalArg<T>
where
    T: ContractCallArg,
{
    #[inline]
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        if let OptionalArg::Some(t) = self {
            t.push_dyn_arg(output);
        }
    }
}

impl<T> ContractCallArg for OptionalArg<T>
where
    T: ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
    }
}

impl<T: TypeAbi> TypeAbi for OptionalArg<T> {
    fn type_name() -> String {
        let mut repr = String::from("optional<");
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
