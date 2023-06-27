use multiversx_chain_vm::mem_conv;
use multiversx_sc::api::{EndpointFinishApi, EndpointFinishApiImpl, HandleConstraints};

use crate::api::{VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> EndpointFinishApi for VMHooksApi<VHB> {
    type EndpointFinishApiImpl = Self;

    fn finish_api_impl() -> Self::EndpointFinishApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> EndpointFinishApiImpl for VMHooksApi<VHB> {
    fn finish_slice_u8(&self, bytes: &[u8]) {
        self.with_vm_hooks(|vh| {
            mem_conv::with_mem_ptr(bytes, |offset, length| {
                vh.finish(offset, length);
            })
        })
    }

    fn finish_big_int_raw(&self, handle: Self::BigIntHandle) {
        self.assert_live_handle(&handle);
        self.with_vm_hooks(|vh| vh.big_int_finish_signed(handle.get_raw_handle_unchecked()));
    }

    fn finish_big_uint_raw(&self, handle: Self::BigIntHandle) {
        self.assert_live_handle(&handle);
        self.with_vm_hooks(|vh| vh.big_int_finish_unsigned(handle.get_raw_handle_unchecked()));
    }

    fn finish_managed_buffer_raw(&self, handle: Self::ManagedBufferHandle) {
        self.assert_live_handle(&handle);
        self.with_vm_hooks(|vh| vh.mbuffer_finish(handle.get_raw_handle_unchecked()));
    }

    fn finish_u64(&self, value: u64) {
        self.with_vm_hooks(|vh| vh.small_int_finish_unsigned(value as i64));
    }

    fn finish_i64(&self, value: i64) {
        self.with_vm_hooks(|vh| vh.small_int_finish_signed(value));
    }
}
