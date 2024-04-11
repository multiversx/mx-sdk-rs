use core::marker::PhantomData;

use crate::api::VMApi;

use super::ContractBase;

/// A unique empty structure that automatically implements all smart contract traits.
///
/// The smart contract macros will automatically also generate trait implementations for this type. These include:
/// - the contract trait
/// - the `AutoImpl` trait
/// - the `EndpointWrappers` trait
///
/// When generating WASM, this contract implementation is used.
/// This makes sure no monomorphization-induced code duplication occurs in relation to modules.
pub struct UniversalContractObj<A>
where
    A: VMApi,
{
    _phantom: PhantomData<A>,
}

impl<A> UniversalContractObj<A>
where
    A: VMApi,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<A> Default for UniversalContractObj<A>
where
    A: VMApi,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<A> ContractBase for UniversalContractObj<A>
where
    A: VMApi,
{
    type Api = A;
}
