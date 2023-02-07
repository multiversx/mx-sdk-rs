use crate::{
    scenario::model::{BytesValue, CheckValue, CheckValueList},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::CheckLogRaw,
    },
};

#[derive(Debug, Clone)]
pub struct CheckLog {
    pub address: CheckValue<BytesValue>,
    pub endpoint: CheckValue<BytesValue>,
    pub topics: CheckValueList,
    pub data: CheckValue<BytesValue>,
}

impl InterpretableFrom<CheckLogRaw> for CheckLog {
    fn interpret_from(from: CheckLogRaw, context: &InterpreterContext) -> Self {
        CheckLog {
            address: CheckValue::<BytesValue>::interpret_from(from.address, context),
            endpoint: CheckValue::<BytesValue>::interpret_from(from.endpoint, context),
            topics: CheckValueList::interpret_from(from.topics, context),
            data: CheckValue::<BytesValue>::interpret_from(from.data, context),
        }
    }
}

impl IntoRaw<CheckLogRaw> for CheckLog {
    fn into_raw(self) -> CheckLogRaw {
        CheckLogRaw {
            address: self.address.into_raw(),
            endpoint: self.endpoint.into_raw(),
            topics: self.topics.into_raw(),
            data: self.data.into_raw(),
        }
    }
}
