use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{AddressValue, BytesValue},
    serde_raw::TxQueryRaw,
};

#[derive(Debug)]
pub struct TxQuery {
    pub to: AddressValue,
    pub function: String,
    pub arguments: Vec<BytesValue>,
}

impl InterpretableFrom<TxQueryRaw> for TxQuery {
    fn interpret_from(from: TxQueryRaw, context: &InterpreterContext) -> Self {
        TxQuery {
            to: AddressValue::interpret_from(from.to, context),
            function: from.function,
            arguments: from
                .arguments
                .into_iter()
                .map(|t| BytesValue::interpret_from(t, context))
                .collect(),
        }
    }
}
