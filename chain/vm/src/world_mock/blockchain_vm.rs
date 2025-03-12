use std::{ops::Deref, sync::Arc};

use multiversx_chain_vm_executor::CompilationOptions;

use crate::builtin_function_mocks::BuiltinFunctionContainer;

pub const COMPILATION_OPTIONS: CompilationOptions = CompilationOptions {
    gas_limit: 1,
    unmetered_locals: 0,
    max_memory_grow: 0,
    max_memory_grow_delta: 0,
    opcode_trace: false,
    metering: false,
    runtime_breakpoints: false,
};

pub struct BlockchainVM {
    pub builtin_functions: BuiltinFunctionContainer,
    pub compilation_options: CompilationOptions,
}

#[derive(Clone, Default)]
pub struct BlockchainVMRef(Arc<BlockchainVM>);

impl Default for BlockchainVM {
    fn default() -> Self {
        Self {
            builtin_functions: Default::default(),
            compilation_options: COMPILATION_OPTIONS,
        }
    }
}

impl BlockchainVM {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BlockchainVMRef {
    pub fn new() -> Self {
        BlockchainVMRef(Arc::new(BlockchainVM::new()))
    }
}

impl Deref for BlockchainVMRef {
    type Target = BlockchainVM;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
