use std::path::PathBuf;

use crate::value_interpreter::VMIdentifier;

#[derive(Default, Clone)]
pub struct InterpreterContext {
    pub context_path: PathBuf,
    pub vm_type: VMIdentifier,
}

impl InterpreterContext {
    pub fn new(context_path: PathBuf, vm_type: VMIdentifier) -> Self {
        InterpreterContext {
            context_path,
            vm_type,
        }
    }

    pub fn with_dir(self, context_path: PathBuf) -> Self {
        InterpreterContext {
            context_path,
            ..self
        }
    }
}

pub trait InterpretableFrom<T> {
    fn interpret_from(from: T, buider: &InterpreterContext) -> Self;
}

impl<T> InterpretableFrom<T> for T {
    fn interpret_from(from: T, _buider: &InterpreterContext) -> Self {
        from
    }
}

impl<T: Clone> InterpretableFrom<&T> for T {
    fn interpret_from(from: &T, _buider: &InterpreterContext) -> Self {
        from.clone()
    }
}

pub trait IntoRaw<R> {
    fn into_raw(self) -> R;
}
