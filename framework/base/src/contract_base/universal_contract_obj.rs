use core::{cell::UnsafeCell, marker::PhantomData};

use crate::api::{const_handles, RawHandle, VMApi};

use super::{CallValueWrapper, ContractBase};

pub struct ContractObjData {
    pub call_value_egld_handle: RawHandle,
    pub call_value_multi_esdt_handle: RawHandle,
}

impl Default for ContractObjData {
    fn default() -> Self {
        ContractObjData {
            call_value_egld_handle: const_handles::UNINITIALIZED_HANDLE,
            call_value_multi_esdt_handle: const_handles::UNINITIALIZED_HANDLE,
        }
    }
}

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
    pub data: UnsafeCell<ContractObjData>,
}

unsafe impl<A> Sync for UniversalContractObj<A> where A: VMApi {}
unsafe impl<A> Send for UniversalContractObj<A> where A: VMApi {}

impl<A> UniversalContractObj<A>
where
    A: VMApi,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: UnsafeCell::new(ContractObjData::default()),
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

    fn call_value(&self) -> CallValueWrapper<'_, Self::Api> {
        CallValueWrapper::new(&self.data)
    }
}
