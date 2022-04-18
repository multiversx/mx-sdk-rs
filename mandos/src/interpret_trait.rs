use std::path::PathBuf;

#[derive(Default)]
pub struct InterpreterContext {
    pub context_path: PathBuf,
}

impl InterpreterContext {
    pub fn new(context_path: PathBuf) -> Self {
        InterpreterContext { context_path }
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
