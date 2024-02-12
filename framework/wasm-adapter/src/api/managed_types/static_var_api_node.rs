use multiversx_sc::{
    api::{const_handles, BigIntApiImpl, RawHandle, StaticVarApi, StaticVarApiImpl},
    types::LockableStaticBuffer,
};

use crate::api::VmApiImpl;

static mut STATIC_BUFFER: LockableStaticBuffer = LockableStaticBuffer::new();
static mut EXTERNAL_VIEW_TARGET_ADDRESS_HANDLE: i32 = 0;
static mut NEXT_HANDLE: i32 = const_handles::NEW_HANDLE_START_FROM;
static mut NUM_ARGUMENTS: i32 = 0;
static mut CALL_VALUE_EGLD_HANDLE: i32 = const_handles::UNINITIALIZED_HANDLE;
static mut CALL_VALUE_MULTI_ESDT_HANDLE: i32 = const_handles::UNINITIALIZED_HANDLE;
static mut SCALING_FACTOR_INIT: [bool; const_handles::SCALING_FACTOR_LENGTH] =
    [false; const_handles::SCALING_FACTOR_LENGTH];

// The compiler seems to enjoy inlining this method no matter how many times it shows up.
// Hence the rather drastic directive.
#[inline(never)]
fn next_handle() -> i32 {
    unsafe {
        NEXT_HANDLE -= 1;
        NEXT_HANDLE
    }
}

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

    fn set_external_view_target_address_handle(&self, handle: RawHandle) {
        unsafe {
            EXTERNAL_VIEW_TARGET_ADDRESS_HANDLE = handle;
        }
    }

    fn get_external_view_target_address_handle(&self) -> RawHandle {
        unsafe { EXTERNAL_VIEW_TARGET_ADDRESS_HANDLE }
    }

    fn next_handle(&self) -> RawHandle {
        next_handle()
    }

    fn set_num_arguments(&self, num_arguments: i32) {
        unsafe {
            NUM_ARGUMENTS = num_arguments;
        }
    }

    fn get_num_arguments(&self) -> i32 {
        unsafe { NUM_ARGUMENTS }
    }

    fn set_call_value_egld_handle(&self, handle: RawHandle) {
        unsafe {
            CALL_VALUE_EGLD_HANDLE = handle;
        }
    }

    fn get_call_value_egld_handle(&self) -> RawHandle {
        unsafe { CALL_VALUE_EGLD_HANDLE }
    }

    fn set_call_value_multi_esdt_handle(&self, handle: RawHandle) {
        unsafe {
            CALL_VALUE_MULTI_ESDT_HANDLE = handle;
        }
    }

    fn get_call_value_multi_esdt_handle(&self) -> RawHandle {
        unsafe { CALL_VALUE_MULTI_ESDT_HANDLE }
    }

    fn set_scaling_factor_cached(&self, decimals: usize) {
        let handle = const_handles::get_scaling_factor_handle(decimals);
        let ten = self.bi_new(10i64);
        let num_decimals = self.bi_new(decimals as i64);
        self.bi_pow(handle, ten, num_decimals);
        self.set_initialized(decimals);
    }

    fn is_scaling_factor_cached(&self, decimals: usize) -> bool {
        unsafe { SCALING_FACTOR_INIT[decimals] }
    }

    fn set_initialized(&self, decimals: usize) {
        {
            unsafe { SCALING_FACTOR_INIT[decimals] = true }
        }
    }
}
