use multiversx_chain_vm::host::vm_hooks::{VMHooksContext, VMHooksDispatcher};
use multiversx_chain_vm_executor::VMHooks;

pub trait VMHooksDebugger: VMHooks {
    fn drop_managed_buffer(&self, handle: i32);
    fn drop_big_float(&self, handle: i32);
    fn drop_big_int(&self, handle: i32);
    fn drop_elliptic_curve(&self, handle: i32);
    fn drop_managed_map(&self, handle: i32);
}

impl<C: VMHooksContext> VMHooksDebugger for VMHooksDispatcher<C> {
    fn drop_managed_buffer(&self, handle: i32) {
        self.get_handler().mb_drop(handle);
    }

    fn drop_big_float(&self, handle: i32) {
        self.get_handler().bf_drop(handle);
    }

    fn drop_big_int(&self, handle: i32) {
        self.get_handler().bi_drop(handle);
    }

    fn drop_elliptic_curve(&self, _handle: i32) {
        // TODO: not implemented
    }

    fn drop_managed_map(&self, handle: i32) {
        self.get_handler().mm_drop(handle);
    }
}
