use crate::scenario_model::BytesValue;

#[derive(Debug, Clone)]
pub struct Log {
    pub address: BytesValue,
    pub endpoint: BytesValue,
    pub topics: Vec<BytesValue>,
    pub data: BytesValue,
}
