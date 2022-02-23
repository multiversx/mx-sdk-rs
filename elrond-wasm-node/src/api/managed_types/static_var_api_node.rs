use elrond_wasm::{
    api::{Handle, StaticVarApi, StaticVarApiImpl},
    types::LockableStaticBuffer,
};

use crate::VmApiImpl;

static mut STATIC_BUFFER: LockableStaticBuffer = LockableStaticBuffer::new();
static mut EXTERNAL_VIEW_TARGET_ADDRESS_HANDLE: i32 = 0;
static mut NEXT_BIG_INT_HANDLE: i32 = <VmApiImpl as StaticVarApi>::BIG_INT_HANDLE_START_FROM;
static mut NEXT_MANAGED_BUFFER_HANDLE: i32 =
    <VmApiImpl as StaticVarApi>::MANAGED_BUFFER_HANDLE_START_FROM;

impl StaticVarApi for VmApiImpl {
    type StaticVarApiImpl = VmApiImpl;

    fn static_var_api_impl() -> Self::StaticVarApiImpl {
        VmApiImpl {}
    }
}

impl StaticVarApiImpl for VmApiImpl {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R {
        unsafe { f(&mut STATIC_BUFFER) }
    }

    fn set_external_view_target_address_handle(&self, handle: Handle) {
        unsafe {
            EXTERNAL_VIEW_TARGET_ADDRESS_HANDLE = handle;
        }
    }

    fn get_external_view_target_address_handle(&self) -> Handle {
        unsafe { EXTERNAL_VIEW_TARGET_ADDRESS_HANDLE }
    }

    fn next_bigint_handle(&self) -> Handle {
        unsafe {
            NEXT_BIG_INT_HANDLE -= 1;
            NEXT_BIG_INT_HANDLE
        }
    }

    fn get_next_managed_buffer_handle(&self) -> Handle {
        unsafe {
            NEXT_MANAGED_BUFFER_HANDLE -= 1;
            NEXT_MANAGED_BUFFER_HANDLE
        }
    }
}
