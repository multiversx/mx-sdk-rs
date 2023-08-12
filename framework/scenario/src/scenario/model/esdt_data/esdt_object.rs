use super::EsdtInstance;
use crate::scenario::model::{BigUintValue, BytesValue, U64Value};

#[derive(Debug, Default, Clone)]
pub struct EsdtObject {
    pub token_identifier: Option<BytesValue>,
    pub instances: Vec<EsdtInstance>,
    pub last_nonce: Option<U64Value>,
    pub roles: Vec<String>,
    pub frozen: Option<U64Value>,
}

impl EsdtObject {
    pub fn is_short_form(&self) -> bool {
        let has_single_fungible_instance =
            self.instances.len() == 1 && self.instances[0].is_simple_fungible();

        has_single_fungible_instance
            && self.token_identifier.is_none()
            && self.last_nonce.is_none()
            && self.roles.is_empty()
            && self.frozen.is_none()
    }

    pub fn set_balance<N, A>(&mut self, token_nonce_expr: N, amount_expr: A)
    where
        U64Value: From<N>,
        BigUintValue: From<A>,
    {
        let amount = BigUintValue::from(amount_expr);
        let inst_for_nonce = self.get_or_insert_instance_for_nonce(token_nonce_expr);

        if amount.value > 0u32.into() {
            inst_for_nonce.balance = Some(amount);
        } else {
            inst_for_nonce.balance = None;
        }
    }

    pub fn set_token_attributes<N, T>(&mut self, token_nonce_expr: N, attributes_expr: T)
    where
        U64Value: From<N>,
        BytesValue: From<T>,
    {
        let attr_bytes = BytesValue::from(attributes_expr);
        let inst_for_nonce = self.get_or_insert_instance_for_nonce(token_nonce_expr);

        if !attr_bytes.value.is_empty() {
            inst_for_nonce.attributes = Some(attr_bytes);
        } else {
            inst_for_nonce.attributes = None;
        }
    }

    pub fn set_last_nonce<N>(&mut self, last_nonce_expr: N)
    where
        U64Value: From<N>,
    {
        let last_nonce = U64Value::from(last_nonce_expr);
        if last_nonce.value > 0 {
            self.last_nonce = Some(last_nonce);
        } else {
            self.last_nonce = None;
        }
    }

    #[inline]
    pub fn set_roles(&mut self, roles: Vec<String>) {
        self.roles = roles;
    }

    pub fn get_or_insert_instance_for_nonce<N>(&mut self, token_nonce_expr: N) -> &mut EsdtInstance
    where
        U64Value: From<N>,
    {
        let token_nonce = U64Value::from(token_nonce_expr);

        if let Some(i) = self.instances.iter().position(|inst| match &inst.nonce {
            Some(nonce) => nonce.value == token_nonce.value,
            None => token_nonce.value == 0,
        }) {
            &mut self.instances[i]
        } else {
            let opt_new_nonce = if token_nonce.value > 0 {
                Some(token_nonce)
            } else {
                None
            };
            let new_inst = EsdtInstance {
                nonce: opt_new_nonce,
                ..Default::default()
            };
            self.instances.push(new_inst);

            let last_index = self.instances.len() - 1;
            &mut self.instances[last_index]
        }
    }
}
