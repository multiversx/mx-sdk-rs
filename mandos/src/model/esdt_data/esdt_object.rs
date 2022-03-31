use crate::model::{BytesValue, U64Value};

use super::EsdtInstance;

#[derive(Debug, Default)]
pub struct EsdtObject {
    pub token_identifier: Option<BytesValue>,
    pub instances: Vec<EsdtInstance>,
    pub last_nonce: Option<U64Value>,
    pub roles: Vec<String>,
    pub frozen: Option<U64Value>,
}
