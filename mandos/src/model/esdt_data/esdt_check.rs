use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    model::BytesKey,
    serde_raw::CheckEsdtRaw,
};

use super::CheckEsdtData;

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

impl IntoRaw<CheckEsdtRaw> for CheckEsdt {
    fn into_raw(self) -> CheckEsdtRaw {
        match self {
            CheckEsdt::Full(m) => CheckEsdtRaw::Full(m.into_raw()),
            CheckEsdt::Short(v) => CheckEsdtRaw::Short(v.into_raw()),
        }
    }
}
