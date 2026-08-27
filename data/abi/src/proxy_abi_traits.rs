use multiversx_sc_codec::TopEncodeMulti;

use crate::{AbiType, AbiTypeFrom, TypeAbi, TypeAbiFrom};

/// Trait that is automatically implemented for all types that are allowed as proxy inputs.
///
/// Is automatically implemented for all traits that are `TypeAbiInto<O> + TopEncodeMulti`.
pub trait ProxyArg<O>: TopEncodeMulti {}

impl<O, T> ProxyArg<O> for T
where
    O: TypeAbiFrom<T>,
    T: TopEncodeMulti,
{
}

pub trait ProxyArg2<O>: TopEncodeMulti {}

impl<O, T> ProxyArg2<O> for T
where
    T: TopEncodeMulti + TypeAbi,
    O: AbiTypeFrom<T::Abi>,
{
}

/// Transaction marker, which indicates that a transaction should never have any payment added to it.
///
/// The implementation is completely identical to the empty payment `()`,
/// the only difference is that the payment methods in `Tx` can only be called on top of `()` payment, not `NotPayable`.
///
/// So basically, `NotPayable` acts as a seal, preventing further payments to be added.
pub struct NotPayable;

/// Trait for adding arguments to a typed call.
/// Since `.argument()` preserves the type, this trait is straightforward to implement.
pub trait ApplyArgument: Sized {
    fn apply_argument<A: TopEncodeMulti>(self, arg: &A) -> Self;
}

/// Defines a proxy object for a smart contract, abstracting away the concrete transaction type.
///
/// Analogous to the framework-level `TxProxyTrait`, but generic over the wrapped transaction
/// base `T` instead of the concrete `Tx` type parameters.
pub trait AbiProxyTrait<T> {
    type Methods;
    fn proxy_methods(self, wrapped_tx: T) -> Self::Methods;
}

/// Combines `.payment(P)`, `.raw_call(name)`, and `.original_result::<O>()` into a single
/// trait method, since both `.payment(P)` and `.original_result::<O>()` change the output type.
pub trait IntoCall<P, O>: Sized {
    type Out: ApplyArgument;
    fn into_call(self, payment: P, function_name: &str) -> Self::Out;
}

/// Combines `.payment(P)`, `.raw_deploy()`, and `.original_result::<O>()` into a single
/// trait method, since all three change the output type.
///
/// Only implemented for transaction bases with no recipient (`To = ()`),
/// since deploys create a new contract address.
pub trait IntoDeploy<P, O>: Sized {
    type Out: ApplyArgument;
    fn into_deploy(self, payment: P) -> Self::Out;
}

/// Combines `.payment(P)`, `.raw_upgrade()`, and `.original_result::<O>()` into a single
/// trait method, since all three change the output type.
pub trait IntoUpgrade<P, O>: Sized {
    type Out: ApplyArgument;
    fn into_upgrade(self, payment: P) -> Self::Out;
}
