use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::{EsdtFullRaw, EsdtRaw},
};

use super::{EsdtInstance, EsdtObject};
use crate::scenario::model::{BigUintValue, BytesValue, U64Value};

#[derive(Debug, Clone)]
pub enum Esdt {
    Short(BigUintValue),
    Full(EsdtObject),
}

impl Esdt {
    pub fn convert_to_short_if_possible(&mut self) {
        if let Esdt::Full(esdt_obj) = self {
            if esdt_obj.is_short_form() {
                *self = Self::Short(esdt_obj.instances[0].balance.clone().unwrap())
            }
        }
    }

    pub fn convert_to_full(&mut self) {
        if let Esdt::Short(balance) = self {
            let mut new_esdt_obj = EsdtObject::default();
            new_esdt_obj.set_balance(0u64, balance.clone());

            *self = Self::Full(new_esdt_obj);
        }
    }

    pub fn set_balance<N, A>(&mut self, token_nonce_expr: N, amount_expr: A)
    where
        U64Value: From<N>,
        BigUintValue: From<A>,
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
                Esdt::Short(BigUintValue::interpret_from(short_esdt, context))
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
    fn into_raw(mut self) -> EsdtRaw {
        self.convert_to_short_if_possible();

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
