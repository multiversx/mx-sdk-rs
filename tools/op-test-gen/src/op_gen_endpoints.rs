use std::fmt::Write;

use crate::{OperatorGroup, OperatorInfo, OperatorList};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    BigInt,
    BigIntRef,
    BigUint,
    BigUintRef,
    NonZeroBigUint,
    NonZeroBigUintRef,
    Usize,
    I32,
    I64,
    U32,
    U64,
    Bool,
}

impl ValueType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValueType::BigInt => "BigInt",
            ValueType::BigIntRef => "&BigInt",
            ValueType::BigUint => "BigUint",
            ValueType::BigUintRef => "&BigUint",
            ValueType::NonZeroBigUint => "NonZeroBigUint",
            ValueType::NonZeroBigUintRef => "&NonZeroBigUint",
            ValueType::Usize => "usize",
            ValueType::I32 => "i32",
            ValueType::I64 => "i64",
            ValueType::U32 => "u32",
            ValueType::U64 => "u64",
            ValueType::Bool => "bool",
        }
    }

    pub fn as_method_name_item(&self) -> &'static str {
        match self {
            ValueType::BigInt => "big_int",
            ValueType::BigIntRef => "big_int_ref",
            ValueType::BigUint => "big_uint",
            ValueType::BigUintRef => "big_uint_ref",
            ValueType::NonZeroBigUint => "non_zero_big_uint",
            ValueType::NonZeroBigUintRef => "non_zero_big_uint_ref",
            _ => self.as_str(),
        }
    }

    pub fn is_signed(self) -> bool {
        matches!(self, ValueType::BigInt | ValueType::BigIntRef)
    }

    pub fn is_big_uint(self) -> bool {
        matches!(self, ValueType::BigUint | ValueType::BigUintRef)
    }

    pub fn is_non_zero(self) -> bool {
        matches!(
            self,
            ValueType::NonZeroBigUint | ValueType::NonZeroBigUintRef
        )
    }
}

pub struct BigNumOperatorTestEndpoint {
    pub fn_name: String,
    pub op_info: OperatorInfo,
    pub a_mut: bool,
    pub a_type: ValueType,
    pub b_type: ValueType,
    pub return_type: ValueType,
    pub body: String,
}

impl BigNumOperatorTestEndpoint {
    pub fn new(
        op_info: &OperatorInfo,
        a_type: ValueType,
        b_type: ValueType,
        return_type: ValueType,
    ) -> Self {
        let body = if op_info.assign {
            format!(
                "
        a {op} b;
        a
    ",
                op = op_info.symbol()
            )
        } else {
            format!(
                "
        a {op} b
    ",
                op = op_info.symbol()
            )
        };

        Self {
            fn_name: format!(
                "{}_{}_{}",
                op_info.name,
                a_type.as_method_name_item(),
                b_type.as_method_name_item()
            ),
            op_info: op_info.clone(),
            a_mut: op_info.assign, // "mut a", for assign operator, so we can change a directly
            a_type,
            b_type,
            return_type,
            body,
        }
    }

    pub fn write_endpoint(&self, out: &mut String) {
        write!(
            out,
            "
    #[endpoint]
    fn {}(&self, {}a: {}, b: {}) -> {} {{{}}}",
            self.fn_name,
            if self.a_mut { "mut " } else { "" },
            self.a_type.as_str(),
            self.b_type.as_str(),
            self.return_type.as_str(),
            self.body
        )
        .unwrap();
    }
}

fn append_all_combinations(
    op: &OperatorInfo,
    type_1: ValueType,
    type_2: ValueType,
    return_type: ValueType,
    endpoints: &mut Vec<BigNumOperatorTestEndpoint>,
) {
    for a_type in [type_1, type_2] {
        for b_type in [type_1, type_2] {
            endpoints.push(BigNumOperatorTestEndpoint::new(
                op,
                a_type,
                b_type,
                return_type,
            ));
        }
    }
}

fn add_u32_u64_endpoints(
    op: &OperatorInfo,
    owned_type: ValueType,
    opt_ref_type: Option<ValueType>,
    endpoints: &mut Vec<BigNumOperatorTestEndpoint>,
) {
    endpoints.push(BigNumOperatorTestEndpoint::new(
        op,
        owned_type,
        ValueType::U32,
        owned_type,
    ));
    if let Some(ref_type) = opt_ref_type {
        endpoints.push(BigNumOperatorTestEndpoint::new(
            op,
            ref_type,
            ValueType::U32,
            owned_type,
        ));
    }
    endpoints.push(BigNumOperatorTestEndpoint::new(
        op,
        owned_type,
        ValueType::U64,
        owned_type,
    ));
    if let Some(ref_type) = opt_ref_type {
        endpoints.push(BigNumOperatorTestEndpoint::new(
            op,
            ref_type,
            ValueType::U64,
            owned_type,
        ));
    }
}

fn add_cmp_small_int_endpoints(
    op: &OperatorInfo,
    owned_type: ValueType,
    endpoints: &mut Vec<BigNumOperatorTestEndpoint>,
) {
    for small_int_type in [
        ValueType::I32,
        ValueType::I64,
        ValueType::U32,
        ValueType::U64,
    ] {
        endpoints.push(BigNumOperatorTestEndpoint::new(
            op,
            owned_type,
            small_int_type,
            ValueType::Bool,
        ));
    }
}

pub fn create_endpoints_for_op(op: &OperatorInfo) -> Vec<BigNumOperatorTestEndpoint> {
    let mut endpoints = Vec::new();

    match op.group {
        OperatorGroup::Arithmetic => {
            if op.assign {
                // Assign operators, +=, -=, etc.
                // They only have the owned type as first argument
                // BigInt
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::BigInt,
                    ValueType::BigInt,
                    ValueType::BigInt,
                ));
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::BigInt,
                    ValueType::BigIntRef,
                    ValueType::BigInt,
                ));
                // BigUint
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::BigUint,
                    ValueType::BigUint,
                    ValueType::BigUint,
                ));
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::BigUint,
                    ValueType::BigUintRef,
                    ValueType::BigUint,
                ));
                add_u32_u64_endpoints(op, ValueType::BigUint, None, &mut endpoints);

                // NonZeroBigUint
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::NonZeroBigUint,
                    ValueType::NonZeroBigUint,
                    ValueType::NonZeroBigUint,
                ));
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::NonZeroBigUint,
                    ValueType::NonZeroBigUintRef,
                    ValueType::NonZeroBigUint,
                ));

                // NonZeroBigUint += BigUint/&BigUint
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::NonZeroBigUint,
                    ValueType::BigUint,
                    ValueType::NonZeroBigUint,
                ));
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::NonZeroBigUint,
                    ValueType::BigUintRef,
                    ValueType::NonZeroBigUint,
                ));

                // NonZeroBigUint += u32/u64
                add_u32_u64_endpoints(op, ValueType::NonZeroBigUint, None, &mut endpoints);
            } else {
                // Direct, non-assign operators, +-*/%
                // BigInt
                append_all_combinations(
                    op,
                    ValueType::BigInt,
                    ValueType::BigIntRef,
                    ValueType::BigInt,
                    &mut endpoints,
                );

                // BigUint
                append_all_combinations(
                    op,
                    ValueType::BigUint,
                    ValueType::BigUintRef,
                    ValueType::BigUint,
                    &mut endpoints,
                );
                add_u32_u64_endpoints(
                    op,
                    ValueType::BigUint,
                    Some(ValueType::BigUintRef),
                    &mut endpoints,
                );

                // NonZeroBigUint
                append_all_combinations(
                    op,
                    ValueType::NonZeroBigUint,
                    ValueType::NonZeroBigUintRef,
                    ValueType::NonZeroBigUint,
                    &mut endpoints,
                );

                add_u32_u64_endpoints(
                    op,
                    ValueType::NonZeroBigUint,
                    Some(ValueType::NonZeroBigUintRef),
                    &mut endpoints,
                );
            }
        }
        OperatorGroup::Bitwise => {
            // Bitwise operators are only defined for BigUint
            if op.assign {
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::BigUint,
                    ValueType::BigUint,
                    ValueType::BigUint,
                ));
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::BigUint,
                    ValueType::BigUintRef,
                    ValueType::BigUint,
                ));

                add_u32_u64_endpoints(op, ValueType::BigUint, None, &mut endpoints);
            } else {
                append_all_combinations(
                    op,
                    ValueType::BigUint,
                    ValueType::BigUintRef,
                    ValueType::BigUint,
                    &mut endpoints,
                );

                add_u32_u64_endpoints(
                    op,
                    ValueType::BigUint,
                    Some(ValueType::BigUintRef),
                    &mut endpoints,
                );
            }
        }
        OperatorGroup::Shift => {
            // Shift operators are only defined for BigUint and usize
            if op.assign {
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::BigUint,
                    ValueType::Usize,
                    ValueType::BigUint,
                ));
            } else {
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::BigUint,
                    ValueType::Usize,
                    ValueType::BigUint,
                ));
                endpoints.push(BigNumOperatorTestEndpoint::new(
                    op,
                    ValueType::BigUintRef,
                    ValueType::Usize,
                    ValueType::BigUint,
                ));
            }
        }
        OperatorGroup::Cmp => {
            endpoints.push(BigNumOperatorTestEndpoint::new(
                op,
                ValueType::BigInt,
                ValueType::BigInt,
                ValueType::Bool,
            ));
            add_cmp_small_int_endpoints(op, ValueType::BigInt, &mut endpoints);
            endpoints.push(BigNumOperatorTestEndpoint::new(
                op,
                ValueType::BigUint,
                ValueType::BigUint,
                ValueType::Bool,
            ));
            add_cmp_small_int_endpoints(op, ValueType::BigUint, &mut endpoints);
            endpoints.push(BigNumOperatorTestEndpoint::new(
                op,
                ValueType::NonZeroBigUint,
                ValueType::NonZeroBigUint,
                ValueType::Bool,
            ));
            add_cmp_small_int_endpoints(op, ValueType::NonZeroBigUint, &mut endpoints);
        }
    }

    endpoints
}

pub fn create_all_endpoints(ops: &OperatorList) -> Vec<BigNumOperatorTestEndpoint> {
    ops.0.iter().flat_map(create_endpoints_for_op).collect()
}
