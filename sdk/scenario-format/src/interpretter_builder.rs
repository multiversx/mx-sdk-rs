use std::path::PathBuf;

use crate::{
    interpret_trait::InterpreterContext,
    serde_raw::ValueSubTree,
    value_interpreter::{interpret_string, interpret_subtree, VMIdentifier},
};

pub struct InterpreterBuilder {
    pub context: InterpreterContext,
}

impl Default for InterpreterBuilder {
    fn default() -> Self {
        Self::new(std::env::current_dir().unwrap(), VMIdentifier::default())
    }
}

impl InterpreterBuilder {
    pub fn new(context_path: PathBuf, vm_type: VMIdentifier) -> Self {
        InterpreterBuilder {
            context: InterpreterContext {
                context_path,
                vm_type,
            },
        }
    }

    pub fn interpret_subtree(self, vst: &ValueSubTree) -> Vec<u8> {
        interpret_subtree(vst, &self.context)
    }

    pub fn interpret_string(self, s: &str) -> Vec<u8> {
        interpret_string(s, &self.context)
    }
}

pub trait InterpretableFrom<T> {
    fn interpret_from(from: T, buider: &InterpreterBuilder) -> Self;
}

impl<T> InterpretableFrom<T> for T {
    fn interpret_from(from: T, _buider: &InterpreterBuilder) -> Self {
        from
    }
}

impl<T: Clone> InterpretableFrom<&T> for T {
    fn interpret_from(from: &T, _buider: &InterpreterBuilder) -> Self {
        from.clone()
    }
}

pub trait IntoRaw<R> {
    fn into_raw(self) -> R;
}
