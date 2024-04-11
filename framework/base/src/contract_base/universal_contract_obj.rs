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
pub struct UniversalContractObj<'a, A>
where
    A: VMApi<'a>,
{
    _phantom: PhantomData<A>,
}

impl<'a, A> UniversalContractObj<'a, A>
where
    A: VMApi<'a>,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'a, A> Default for UniversalContractObj<'a, A>
where
    A: VMApi<'a>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, A> ContractBase<'a> for UniversalContractObj<'a, A>
where
    A: VMApi<'a>,
{
    type Api = A;
}
