use std::{ops::Deref, sync::Arc};

use multiversx_chain_vm_executor::CompilationOptions;

use crate::{builtin_functions::BuiltinFunctionContainer, schedule::GasSchedule};

pub const COMPILATION_OPTIONS: CompilationOptions = CompilationOptions {
    gas_limit: 1,
    unmetered_locals: 0,
    max_memory_grow: 0,
    max_memory_grow_delta: 0,
    opcode_trace: false,
    metering: false,
    runtime_breakpoints: false,
};

pub const COMPILATION_OPTIONS_GAS: CompilationOptions = CompilationOptions {
    gas_limit: 600_000_000u64,
    unmetered_locals: 0,
    max_memory_grow: 0,
    max_memory_grow_delta: 0,
    opcode_trace: false,
    metering: true,
    runtime_breakpoints: false,
};

pub struct VMConfig {
    pub builtin_functions: BuiltinFunctionContainer,
    pub compilation_options: CompilationOptions,
    pub gas_schedule: GasSchedule,
}

#[derive(Clone, Default)]
pub struct VMConfigRef(Arc<VMConfig>);

impl Default for VMConfig {
    fn default() -> Self {
        Self {
            builtin_functions: Default::default(),
            compilation_options: COMPILATION_OPTIONS,
            gas_schedule: GasSchedule::default(),
        }
    }
}

impl VMConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_gas(gas_schedule: GasSchedule) -> Self {
        Self {
            builtin_functions: Default::default(),
            compilation_options: COMPILATION_OPTIONS_GAS,
            gas_schedule,
        }
    }
}

impl VMConfigRef {
    pub fn new() -> Self {
        VMConfigRef(Arc::new(VMConfig::new()))
    }

    pub fn change_gas_schedule(&mut self, gas_schedule: GasSchedule) {
        let vm_config =
            Arc::get_mut(&mut self.0).expect("cannot change gas schedule during execution");
        vm_config.gas_schedule = gas_schedule;
    }
}

impl Deref for VMConfigRef {
    type Target = VMConfig;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
