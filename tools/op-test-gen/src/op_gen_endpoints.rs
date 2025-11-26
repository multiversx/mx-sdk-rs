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
    U32,
    U64,
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
            ValueType::U32 => "u32",
            ValueType::U64 => "u64",
        }
    }

    pub fn is_signed(self) -> bool {
        matches!(self, ValueType::BigInt | ValueType::BigIntRef)
    }
}

pub struct BigNumOperatorTestEndpoint {
    pub fn_name: String,
    pub op_info: OperatorInfo,
    pub a_type: ValueType,
    pub b_type: ValueType,
    pub return_type: ValueType,
    pub body: String,
}

impl BigNumOperatorTestEndpoint {
    pub fn new(
        fn_name: &str,
        op_info: &OperatorInfo,
        a_type: ValueType,
        b_type: ValueType,
        return_type: ValueType,
    ) -> Self {
        let body = if op_info.assign {
            format!(
                "
        let mut r = a.clone();
        r {op} b;
        r
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
            fn_name: fn_name.to_string(),
            op_info: op_info.clone(),
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
    fn {}(&self, a: {}, b: {}) -> {} {{{}}}",
            self.fn_name,
            self.a_type.as_str(),
            self.b_type.as_str(),
            self.return_type.as_str(),
            self.body
        )
        .unwrap();
    }
}

pub fn create_endpoints_for_op(op: &OperatorInfo) -> Vec<BigNumOperatorTestEndpoint> {
    let mut endpoints = Vec::new();

    if op.group == OperatorGroup::Arithmetic {
        // Binary operator endpoint
        endpoints.push(BigNumOperatorTestEndpoint::new(
            &format!("{}_big_int", op.name),
            op,
            ValueType::BigInt,
            ValueType::BigInt,
            ValueType::BigInt,
        ));
        endpoints.push(BigNumOperatorTestEndpoint::new(
            &format!("{}_big_int_ref", op.name),
            op,
            ValueType::BigIntRef,
            ValueType::BigIntRef,
            ValueType::BigInt,
        ));
    }

    if op.group == OperatorGroup::Shift {
        endpoints.push(BigNumOperatorTestEndpoint::new(
            &format!("{}_big_uint", op.name),
            op,
            ValueType::BigUint,
            ValueType::Usize,
            ValueType::BigUint,
        ));
        endpoints.push(BigNumOperatorTestEndpoint::new(
            &format!("{}_big_uint_ref", op.name),
            op,
            ValueType::BigUintRef,
            ValueType::Usize,
            ValueType::BigUint,
        ));
    } else {
        endpoints.push(BigNumOperatorTestEndpoint::new(
            &format!("{}_big_uint", op.name),
            op,
            ValueType::BigUint,
            ValueType::BigUint,
            ValueType::BigUint,
        ));
        endpoints.push(BigNumOperatorTestEndpoint::new(
            &format!("{}_big_uint_ref", op.name),
            op,
            ValueType::BigUintRef,
            ValueType::BigUintRef,
            ValueType::BigUint,
        ));
    }

    endpoints
}

pub fn create_all_endpoints(ops: &OperatorList) -> Vec<BigNumOperatorTestEndpoint> {
    ops.0.iter().flat_map(create_endpoints_for_op).collect()
}
