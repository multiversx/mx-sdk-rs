use multiversx_chain_vm_executor::VMHooks;

pub trait CleanableVMHooks: VMHooks {
    fn remove_managed_buffer(&self, handle: i32);
}