use multiversx_chain_vm::host::vm_hooks::{VMHooksContext, VMHooksDispatcher};
use multiversx_chain_vm_executor::{VMHooks, VMHooksEarlyExit};

pub trait VMHooksDebugger: VMHooks {
    fn drop_managed_buffer(&self, handle: i32) -> Result<(), VMHooksEarlyExit>;
    fn drop_big_float(&self, handle: i32) -> Result<(), VMHooksEarlyExit>;
    fn drop_big_int(&self, handle: i32) -> Result<(), VMHooksEarlyExit>;
    fn drop_elliptic_curve(&self, handle: i32) -> Result<(), VMHooksEarlyExit>;
    fn drop_managed_map(&self, handle: i32) -> Result<(), VMHooksEarlyExit>;
}

impl<C: VMHooksContext> VMHooksDebugger for VMHooksDispatcher<C> {
    fn drop_managed_buffer(&self, handle: i32) -> Result<(), VMHooksEarlyExit> {
        self.handler.mb_drop(handle);
        Ok(())
    }

    fn drop_big_float(&self, handle: i32) -> Result<(), VMHooksEarlyExit> {
        self.handler.bf_drop(handle);
        Ok(())
    }

    fn drop_big_int(&self, handle: i32) -> Result<(), VMHooksEarlyExit> {
        self.handler.bi_drop(handle);
        Ok(())
    }

    fn drop_elliptic_curve(&self, _handle: i32) -> Result<(), VMHooksEarlyExit> {
        // TODO: not implemented
        Ok(())
    }

    fn drop_managed_map(&self, handle: i32) -> Result<(), VMHooksEarlyExit> {
        self.handler.mm_drop(handle);
        Ok(())
    }
}
