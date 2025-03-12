use std::{ops::Deref, sync::Arc};

use multiversx_chain_vm_executor::CompilationOptions;

use crate::builtin_functions::BuiltinFunctionContainer;

pub const COMPILATION_OPTIONS: CompilationOptions = CompilationOptions {
    gas_limit: 1,
    unmetered_locals: 0,
    max_memory_grow: 0,
    max_memory_grow_delta: 0,
    opcode_trace: false,
    metering: false,
    runtime_breakpoints: false,
};

pub struct VMConfig {
    pub builtin_functions: BuiltinFunctionContainer,
    pub compilation_options: CompilationOptions,
}

#[derive(Clone, Default)]
pub struct VMConfigRef(Arc<VMConfig>);

impl Default for VMConfig {
    fn default() -> Self {
        Self {
            builtin_functions: Default::default(),
            compilation_options: COMPILATION_OPTIONS,
        }
    }
}

impl VMConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl VMConfigRef {
    pub fn new() -> Self {
        VMConfigRef(Arc::new(VMConfig::new()))
    }
}

impl Deref for VMConfigRef {
    type Target = VMConfig;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
