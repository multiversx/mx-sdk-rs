use crate::{
    scenario::model::{AddressValue, BytesValue},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::TxQueryRaw,
    },
};

#[derive(Debug, Default, Clone)]
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

impl IntoRaw<TxQueryRaw> for TxQuery {
    fn into_raw(self) -> TxQueryRaw {
        TxQueryRaw {
            to: self.to.into_raw(),
            function: self.function,
            arguments: self
                .arguments
                .into_iter()
                .map(|arg| arg.into_raw())
                .collect(),
        }
    }
}
