use multiversx_chain_vm_executor::Executor;

/// Dummy executor that fails whenever called.
///
/// Used in dummy contexts.
///
/// TODO: either remove, or move to vm-executor repo.
pub struct FailingExecutor;

impl Executor for FailingExecutor {
    fn new_instance(
        &self,
        _wasm_bytes: &[u8],
        _compilation_options: &multiversx_chain_vm_executor::CompilationOptions,
    ) -> Result<
        Box<dyn multiversx_chain_vm_executor::Instance>,
        multiversx_chain_vm_executor::ExecutorError,
    > {
        panic!("called FailingExecutor")
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &multiversx_chain_vm_executor::CompilationOptions,
    ) -> Result<
        Box<dyn multiversx_chain_vm_executor::Instance>,
        multiversx_chain_vm_executor::ExecutorError,
    > {
        panic!("called FailingExecutor")
    }
}
