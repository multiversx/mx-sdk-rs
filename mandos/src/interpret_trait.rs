#[derive(Default)]
pub struct InterpreterContext {}

pub trait InterpretableFrom<T> {
    fn interpret_from(from: T, context: &InterpreterContext) -> Self;
}
