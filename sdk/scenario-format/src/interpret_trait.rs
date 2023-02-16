use std::path::PathBuf;

use crate::{interpretter_builder::InterpreterBuilder, value_interpreter::VMIdentifier};

pub struct InterpreterContext {
    pub context_path: PathBuf,
    pub vm_type: VMIdentifier,
}

impl InterpreterContext {
    pub fn builder() -> InterpreterBuilder {
        InterpreterBuilder::default()
    }
}
