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
