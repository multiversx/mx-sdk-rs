use crate::{
    model::value_key_bytes::BytesKey, CheckEsdtRaw, InterpretableFrom, InterpreterContext,
};

use super::*;

#[derive(Debug)]
pub enum CheckEsdt {
    Short(BytesKey),
    Full(CheckEsdtData),
}

impl InterpretableFrom<CheckEsdtRaw> for CheckEsdt {
    fn interpret_from(from: CheckEsdtRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckEsdtRaw::Full(m) => CheckEsdt::Full(CheckEsdtData::interpret_from(m, context)),
            CheckEsdtRaw::Short(v) => CheckEsdt::Short(BytesKey::interpret_from(v, context)),
        }
    }
}
