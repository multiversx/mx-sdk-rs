use std::path::Path;

#[derive(Default)]
pub struct InterpreterContext {
    pub context_path: String,
}

impl InterpreterContext {
    pub fn new(path: &Path) -> Self {
        let context_path = path.parent().unwrap().to_str().unwrap().to_string();
        InterpreterContext { context_path }
    }
}

pub trait InterpretableFrom<T> {
    fn interpret_from(from: T, context: &InterpreterContext) -> Self;
}
