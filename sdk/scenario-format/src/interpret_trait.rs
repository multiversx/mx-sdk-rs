use std::path::PathBuf;

use crate::value_interpreter::VMIdentifier;

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
}

impl Default for InterpreterContext {
    fn default() -> Self {
        Self::new(std::env::current_dir().unwrap(), VMIdentifier::default())
    }
}

pub trait InterpretableFrom<T> {
    fn interpret_from(from: T, context: &InterpreterContext) -> Self;
}

impl<T> InterpretableFrom<T> for T {
    fn interpret_from(from: T, _context: &InterpreterContext) -> Self {
        from
    }
}

impl<T: Clone> InterpretableFrom<&T> for T {
    fn interpret_from(from: &T, _context: &InterpreterContext) -> Self {
        from.clone()
    }
}

pub trait IntoRaw<R> {
    fn into_raw(self) -> R;
}
