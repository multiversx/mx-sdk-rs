use super::{CheckEsdtData, CheckEsdtInstance, CheckEsdtInstances};
use crate::{
    scenario::model::{BigUintValue, CheckValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::{CheckEsdtRaw, ValueSubTree},
    },
    scenario_model::BytesValue,
};
use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub enum CheckEsdt {
    Short(BigUintValue),
    Full(CheckEsdtData),
}

impl CheckEsdt {
    pub fn convert_to_short_if_possible(&mut self) {
        if let CheckEsdt::Full(esdt_check) = self {
            let has_single_fungible_instance =
                if let CheckEsdtInstances::Equal(check_instance) = &esdt_check.instances {
                    check_instance.len() == 1 && check_instance[0].is_simple_fungible()
                } else {
                    false
                };

            if has_single_fungible_instance
                && esdt_check.frozen.is_star()
                && esdt_check.last_nonce.is_star()
            {
                let balance =
                    if let CheckEsdtInstances::Equal(check_instances) = &esdt_check.instances {
                        match &check_instances[0].balance {
                            CheckValue::Star => BigUintValue {
                                original: ValueSubTree::Str("*".to_string()),
                                value: BigUint::from(0u32),
                            },
                            CheckValue::Equal(val) => val.clone(),
                        }
                    } else {
                        unreachable!();
                    };

                *self = CheckEsdt::Short(balance);
            }
        }
    }

    pub fn convert_to_full(&mut self) {
        if let CheckEsdt::Short(prev_balance_check) = self {
            let new_instances_check = vec![CheckEsdtInstance {
                balance: CheckValue::Equal(prev_balance_check.clone()),
                ..Default::default()
            }];

            let new_esdt_check = CheckEsdtData {
                instances: CheckEsdtInstances::Equal(new_instances_check),
                ..Default::default()
            };
            *self = CheckEsdt::Full(new_esdt_check);
        }
    }

    pub fn add_balance_check<N, V>(&mut self, nonce_expr: N, balance_expr: V)
    where
        U64Value: InterpretableFrom<N>,
        BigUintValue: InterpretableFrom<V>,
    {
        let ctx = InterpreterContext::default();
        let nonce = U64Value::interpret_from(nonce_expr, &ctx);
        let balance = BigUintValue::interpret_from(balance_expr, &ctx);

        self.convert_to_full();

        if let CheckEsdt::Full(prev_esdt_check) = self {
            match &mut prev_esdt_check.instances {
                CheckEsdtInstances::Star => {
                    let new_instances_check = vec![CheckEsdtInstance {
                        nonce,
                        balance: CheckValue::Equal(balance),
                        ..Default::default()
                    }];

                    prev_esdt_check.instances = CheckEsdtInstances::Equal(new_instances_check);
                },
                CheckEsdtInstances::Equal(esdt_instance_check) => {
                    if let Some(i) = esdt_instance_check
                        .iter()
                        .position(|item| item.nonce.value == nonce.value)
                    {
                        esdt_instance_check[i].balance = CheckValue::Equal(balance);
                    } else {
                        esdt_instance_check.push(CheckEsdtInstance {
                            nonce,
                            balance: CheckValue::Equal(balance),
                            ..Default::default()
                        });
                    }
                },
            }
        }
    }

    pub fn add_balance_and_attributes_check<N, V, T>(
        &mut self,
        nonce_expr: N,
        balance_expr: V,
        attributes_expr: T,
    ) where
        U64Value: From<N>,
        BigUintValue: From<V>,
        BytesValue: From<T>,
    {
        let nonce = U64Value::from(nonce_expr);
        let balance = BigUintValue::from(balance_expr);
        let attributes = BytesValue::from(attributes_expr);

        self.convert_to_full();

        if let CheckEsdt::Full(prev_esdt_check) = self {
            match &mut prev_esdt_check.instances {
                CheckEsdtInstances::Star => {
                    let new_instances_check = vec![CheckEsdtInstance {
                        nonce,
                        balance: CheckValue::Equal(balance),
                        attributes: CheckValue::Equal(attributes),
                        ..Default::default()
                    }];

                    prev_esdt_check.instances = CheckEsdtInstances::Equal(new_instances_check);
                },
                CheckEsdtInstances::Equal(esdt_instance_check) => {
                    if let Some(i) = esdt_instance_check
                        .iter()
                        .position(|item| item.nonce.value == nonce.value)
                    {
                        esdt_instance_check[i].balance = CheckValue::Equal(balance);
                        esdt_instance_check[i].attributes = CheckValue::Equal(attributes);
                    } else {
                        esdt_instance_check.push(CheckEsdtInstance {
                            nonce,
                            balance: CheckValue::Equal(balance),
                            attributes: CheckValue::Equal(attributes),
                            ..Default::default()
                        });
                    }
                },
            }
        }
    }
}

impl InterpretableFrom<CheckEsdtRaw> for CheckEsdt {
    fn interpret_from(from: CheckEsdtRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckEsdtRaw::Full(m) => CheckEsdt::Full(CheckEsdtData::interpret_from(m, context)),
            CheckEsdtRaw::Short(v) => CheckEsdt::Short(BigUintValue::interpret_from(v, context)),
        }
    }
}

impl IntoRaw<CheckEsdtRaw> for CheckEsdt {
    fn into_raw(mut self) -> CheckEsdtRaw {
        self.convert_to_short_if_possible();

        match self {
            CheckEsdt::Full(m) => CheckEsdtRaw::Full(m.into_raw()),
            CheckEsdt::Short(v) => CheckEsdtRaw::Short(v.into_raw()),
        }
    }
}
