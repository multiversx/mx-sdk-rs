use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{BytesKey, BytesValue, U64Value},
    serde_raw::EsdtRaw,
};

use super::{EsdtObject, Instance};

#[derive(Debug)]
pub enum Esdt {
    Short(BytesKey),
    Full(EsdtObject),
}

impl InterpretableFrom<EsdtRaw> for Esdt {
    fn interpret_from(from: EsdtRaw, context: &InterpreterContext) -> Self {
        match from {
            EsdtRaw::Short(short_esdt) => {
                Esdt::Short(BytesKey::interpret_from(short_esdt, context))
            },
            EsdtRaw::Full(full_esdt) => Esdt::Full(EsdtObject {
                token_identifier: full_esdt
                    .token_identifier
                    .map(|b| BytesValue::interpret_from(b, context)),
                instances: full_esdt
                    .instances
                    .into_iter()
                    .map(|instance| Instance::interpret_from(instance, context))
                    .collect(),
                last_nonce: full_esdt
                    .last_nonce
                    .map(|b| U64Value::interpret_from(b, context)),
                roles: full_esdt
                    .roles
                    .into_iter()
                    .map(|role| BytesValue::interpret_from(role, context))
                    .collect(),
                frozen: full_esdt
                    .frozen
                    .map(|b| U64Value::interpret_from(b, context)),
            }),
        }
    }
}
