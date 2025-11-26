use std::fmt::Write;

use crate::{OperatorGroup, OperatorInfo, OperatorList};

pub struct BigNumOperatorTestEndpoint {
    pub fn_name: String,
    pub op_info: OperatorInfo,
    pub a_type: String,
    pub b_type: String,
    pub return_type: String,
    pub body: String,
}

impl BigNumOperatorTestEndpoint {
    pub fn new(
        fn_name: &str,
        op_info: &OperatorInfo,
        a_type: &str,
        b_type: &str,
        return_type: &str,
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
            a_type: a_type.to_string(),
            b_type: b_type.to_string(),
            return_type: return_type.to_string(),
            body,
        }
    }

    pub fn write_endpoint(&self, out: &mut String) {
        write!(
            out,
            "
    #[endpoint]
    fn {}(&self, a: {}, b: {}) -> {} {{{}}}",
            self.fn_name, self.a_type, self.b_type, self.return_type, self.body
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
            "BigInt",
            "BigInt",
            "BigInt",
        ));
        endpoints.push(BigNumOperatorTestEndpoint::new(
            &format!("{}_big_int_ref", op.name),
            op,
            "&BigInt",
            "&BigInt",
            "BigInt",
        ));
    }

    if op.group == OperatorGroup::Shift {
        endpoints.push(BigNumOperatorTestEndpoint::new(
            &format!("{}_big_uint", op.name),
            op,
            "BigUint",
            "usize",
            "BigUint",
        ));
        endpoints.push(BigNumOperatorTestEndpoint::new(
            &format!("{}_big_uint_ref", op.name),
            op,
            "&BigUint",
            "usize",
            "BigUint",
        ));
    } else {
        endpoints.push(BigNumOperatorTestEndpoint::new(
            &format!("{}_big_uint", op.name),
            op,
            "BigUint",
            "BigUint",
            "BigUint",
        ));
        endpoints.push(BigNumOperatorTestEndpoint::new(
            &format!("{}_big_uint_ref", op.name),
            op,
            "&BigUint",
            "&BigUint",
            "BigUint",
        ));
    }

    endpoints
}

pub fn create_all_endpoints(ops: &OperatorList) -> Vec<BigNumOperatorTestEndpoint> {
    ops.0.iter().flat_map(create_endpoints_for_op).collect()
}
