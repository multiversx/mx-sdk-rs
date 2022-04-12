use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    model::{BytesKey, BytesValue, U64Value},
    serde_raw::{EsdtFullRaw, EsdtRaw},
};

use super::{EsdtInstance, EsdtObject};

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
                    .map(|instance| EsdtInstance::interpret_from(instance, context))
                    .collect(),
                last_nonce: full_esdt
                    .last_nonce
                    .map(|b| U64Value::interpret_from(b, context)),
                roles: full_esdt.roles,
                frozen: full_esdt
                    .frozen
                    .map(|b| U64Value::interpret_from(b, context)),
            }),
        }
    }
}

impl IntoRaw<EsdtRaw> for Esdt {
    fn into_raw(self) -> EsdtRaw {
        match self {
            Esdt::Short(short) => EsdtRaw::Short(short.original),
            Esdt::Full(eo) => EsdtRaw::Full(EsdtFullRaw {
                token_identifier: eo.token_identifier.map(|ti| ti.original),
                instances: eo
                    .instances
                    .into_iter()
                    .map(|inst| inst.into_raw())
                    .collect(),
                last_nonce: eo.last_nonce.map(|ti| ti.original),
                roles: eo.roles,
                frozen: eo.frozen.map(|ti| ti.original),
            }),
        }
    }
}
