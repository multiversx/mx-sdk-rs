use super::VMHooksApiBackend;

use std::marker::PhantomData;

use multiversx_chain_vm::{
    executor::{MemPtr, VMHooks},
    tx_mock::StaticVarData,
};
use multiversx_sc::api::{HandleTypeInfo, ManagedBufferApiImpl, RawHandle};

#[derive(Clone, Debug)]
pub struct VMHooksApi<S: VMHooksApiBackend> {
    _phantom: PhantomData<S>,
}

impl<VHB: VMHooksApiBackend> HandleTypeInfo for VMHooksApi<VHB> {
    type ManagedBufferHandle = RawHandle;
    type BigIntHandle = RawHandle;
    type BigFloatHandle = RawHandle;
    type EllipticCurveHandle = RawHandle;
    type ManagedMapHandle = RawHandle;
}

impl<VHB: VMHooksApiBackend> VMHooksApi<VHB> {
    pub fn api_impl() -> VMHooksApi<VHB> {
        VMHooksApi {
            _phantom: PhantomData,
        }
    }

    /// All communication with the VM happens via this method.
    pub fn with_vm_hooks<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        VHB::with_vm_hooks(f)
    }

    /// Static data does not belong to the VM, or to the VM hooks. It belongs to the contract only.
    pub fn with_static_data<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&StaticVarData) -> R,
    {
        VHB::with_static_data(f)
    }

    /// Convenience method for calling VM hooks with a pointer to a temporary buffer in which we load a managed buffer.
    ///
    /// It is used for
    /// - addresses
    /// - token identifiers.
    ///
    /// The buffer is 32 bytes long, enough for both addresses and token identifiers.
    pub(crate) fn with_temp_buffer_ptr<R, F>(&self, handle: RawHandle, length: usize, f: F) -> R
    where
        F: FnOnce(MemPtr) -> R,
    {
        let mut temp_buffer = [0u8; 32];
        self.mb_load_slice(handle, 0, &mut temp_buffer[..length])
            .expect("error extracting address bytes");
        f(temp_buffer.as_ptr() as MemPtr)
    }

    /// Convenience method for calling VM hooks with a pointer to a temporary buffer in which we load an address.
    pub(crate) fn with_temp_address_ptr<R, F>(&self, handle: RawHandle, f: F) -> R
    where
        F: FnOnce(MemPtr) -> R,
    {
        self.with_temp_buffer_ptr(handle, 32, f)
    }
}

pub(crate) fn i32_to_bool(vm_hooks_result: i32) -> bool {
    vm_hooks_result > 0
}

impl<VHB: VMHooksApiBackend> PartialEq for VMHooksApi<VHB> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<VHB: VMHooksApiBackend> Eq for VMHooksApi<VHB> {}
