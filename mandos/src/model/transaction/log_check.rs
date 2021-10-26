use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{BytesValue, CheckValue},
    serde_raw::CheckLogRaw,
};

#[derive(Debug)]
pub struct CheckLog {
    pub address: BytesValue,
    pub endpoint: CheckValue<BytesValue>,
    pub topics: Vec<CheckValue<BytesValue>>,
    pub data: CheckValue<BytesValue>,
}

impl InterpretableFrom<CheckLogRaw> for CheckLog {
    fn interpret_from(from: CheckLogRaw, context: &InterpreterContext) -> Self {
        CheckLog {
            address: BytesValue::interpret_from(from.address, context),
            endpoint: CheckValue::<BytesValue>::interpret_from(from.endpoint, context),
            topics: from
                .topics
                .into_iter()
                .map(|t| CheckValue::<BytesValue>::interpret_from(t, context))
                .collect(),
            data: CheckValue::<BytesValue>::interpret_from(from.data, context),
        }
    }
}
