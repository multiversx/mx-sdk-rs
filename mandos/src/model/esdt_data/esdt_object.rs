use crate::model::{BytesValue, U64Value};

use super::Instance;

#[derive(Debug, Default)]
pub struct EsdtObject {
    pub token_identifier: Option<BytesValue>,
    pub instances: Vec<Instance>,
    pub last_nonce: Option<U64Value>,
    pub roles: Vec<BytesValue>,
    pub frozen: Option<U64Value>,
}
