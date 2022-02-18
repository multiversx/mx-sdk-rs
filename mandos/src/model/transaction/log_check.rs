use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{BytesValue, CheckValue, CheckValueList},
    serde_raw::CheckLogRaw,
};

#[derive(Debug)]
pub struct CheckLog {
    pub address: BytesValue,
    pub endpoint: CheckValue<BytesValue>,
    pub topics: CheckValueList,
    pub data: CheckValue<BytesValue>,
}

impl InterpretableFrom<CheckLogRaw> for CheckLog {
    fn interpret_from(from: CheckLogRaw, context: &InterpreterContext) -> Self {
        CheckLog {
            address: BytesValue::interpret_from(from.address, context),
            endpoint: CheckValue::<BytesValue>::interpret_from(from.endpoint, context),
            topics: CheckValueList::interpret_from(from.topics, context),
            data: CheckValue::<BytesValue>::interpret_from(from.data, context),
        }
    }
}
