use std::path::PathBuf;

use crate::value_interpreter::VMIdentifier;

#[derive(Default, Clone)]
pub struct InterpreterContext {
    pub context_path: PathBuf,
    pub vm_type: VMIdentifier,
    pub allow_missing_files: bool,
}

impl InterpreterContext {
    pub fn new() -> Self {
        InterpreterContext::default()
    }

    pub fn with_dir(self, context_path: PathBuf) -> Self {
        InterpreterContext {
            context_path,
            ..self
        }
    }

    pub fn with_allowed_missing_files(self) -> Self {
        InterpreterContext {
            allow_missing_files: true,
            ..self
        }
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
