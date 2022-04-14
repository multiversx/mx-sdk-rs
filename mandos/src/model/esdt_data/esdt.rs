use num_bigint::BigUint;

use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    model::{BigUintValue, BytesKey, BytesValue, U64Value},
    serde_raw::{EsdtFullRaw, EsdtRaw, ValueSubTree},
};

use super::{EsdtInstance, EsdtObject};

#[derive(Debug)]
pub enum Esdt {
    Short(BytesKey),
    Full(EsdtObject),
}

impl Esdt {
    pub fn convert_to_full(&mut self) {
        if let Esdt::Short(balance_bytes) = self {
            let balance_obj = BigUintValue {
                original: ValueSubTree::Str(balance_bytes.original.clone()),
                value: BigUint::from_bytes_be(&balance_bytes.value),
            };
            let mut new_esdt_obj = EsdtObject::default();
            new_esdt_obj.set_balance(0u64, balance_obj);

            *self = Self::Full(new_esdt_obj);
        }
    }

    pub fn set_balance<N, A>(&mut self, token_nonce_expr: N, amount_expr: A)
    where
        U64Value: InterpretableFrom<N>,
        BigUintValue: InterpretableFrom<A>,
    {
        self.convert_to_full();

        if let Esdt::Full(esdt_obj) = self {
            esdt_obj.set_balance(token_nonce_expr, amount_expr);
        }
    }

    pub fn get_mut_esdt_object(&mut self) -> &mut EsdtObject {
        self.convert_to_full();

        if let Esdt::Full(esdt_obj) = self {
            return esdt_obj;
        }

        unreachable!()
    }
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
