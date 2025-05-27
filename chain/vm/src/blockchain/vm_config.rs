use std::{ops::Deref, sync::Arc};

use crate::{builtin_functions::BuiltinFunctionContainer, schedule::GasSchedule};

#[derive(Default)]
pub struct VMConfig {
    pub builtin_functions: BuiltinFunctionContainer,
    pub gas_schedule: GasSchedule,
}

#[derive(Clone, Default)]
pub struct VMConfigRef(Arc<VMConfig>);

impl VMConfig {
    pub fn new() -> Self {
        Self::default()
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
