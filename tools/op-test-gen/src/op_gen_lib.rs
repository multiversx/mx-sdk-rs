use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct OpInfo {
    pub name: String,
    pub operator: String,
    pub assign: bool,
    pub group: OpGroup,
}

impl OpInfo {
    pub fn new(name: &str, operator: &str, group: OpGroup) -> Self {
        Self {
            name: name.to_owned(),
            operator: operator.to_owned(),
            assign: false,
            group,
        }
    }

    pub fn assign(self) -> Self {
        assert!(!self.assign, "Operator is already an assign operator");
        Self {
            name: format!("{}_assign", self.name),
            operator: format!("{}=", self.operator),
            assign: true,
            group: self.group,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpGroup {
    Arithmetic,
    Bitwise,
    Shift,
}

pub struct OperatorList(Vec<OpInfo>);

impl OperatorList {
    fn create() -> Self {
        let binary_operators = vec![
            // Arithmetic binary operators
            OpInfo::new("add", "+", OpGroup::Arithmetic),
            OpInfo::new("sub", "-", OpGroup::Arithmetic),
            OpInfo::new("mul", "*", OpGroup::Arithmetic),
            OpInfo::new("div", "/", OpGroup::Arithmetic),
            OpInfo::new("rem", "%", OpGroup::Arithmetic),
            // Bitwise binary operators
            OpInfo::new("bit_and", "&", OpGroup::Bitwise),
            OpInfo::new("bit_or", "|", OpGroup::Bitwise),
            OpInfo::new("bit_xor", "^", OpGroup::Bitwise),
            // Bitwise shift binary operators
            OpInfo::new("shr", ">>", OpGroup::Shift),
            OpInfo::new("shl", "<<", OpGroup::Shift),
        ];

        let mut all_operators = Vec::new();
        all_operators.extend(binary_operators.iter().cloned());
        all_operators.extend(binary_operators.iter().cloned().map(|op| op.assign()));
        OperatorList(all_operators)
    }
}

pub struct BigNumOperatorTestEndpoint {
    pub fn_name: String,
    pub op_info: OpInfo,
    pub a_type: String,
    pub b_type: String,
    pub return_type: String,
    pub body: String,
}

impl BigNumOperatorTestEndpoint {
    pub fn new_bin(
        fn_name: &str,
        op_info: &OpInfo,
        a_type: &str,
        b_type: &str,
        return_type: &str,
    ) -> Self {
        let body = if !op_info.assign {
            format!(
                "
        a {op} b
    ",
                op = op_info.operator
            )
        } else {
            format!(
                "
        let mut r = a.clone();
        r {op} b;
        r
    ",
                op = op_info.operator
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
}

pub fn create_endpoints(op: &OpInfo) -> Vec<BigNumOperatorTestEndpoint> {
    let mut endpoints = Vec::new();

    if op.group == OpGroup::Arithmetic {
        // Binary operator endpoint
        endpoints.push(BigNumOperatorTestEndpoint::new_bin(
            &format!("{}_big_int", op.name),
            op,
            "BigInt",
            "BigInt",
            "BigInt",
        ));
        endpoints.push(BigNumOperatorTestEndpoint::new_bin(
            &format!("{}_big_int_ref", op.name),
            op,
            "&BigInt",
            "&BigInt",
            "BigInt",
        ));
    }

    if op.group == OpGroup::Shift {
        endpoints.push(BigNumOperatorTestEndpoint::new_bin(
            &format!("{}_big_uint", op.name),
            op,
            "BigUint",
            "usize",
            "BigUint",
        ));
        endpoints.push(BigNumOperatorTestEndpoint::new_bin(
            &format!("{}_big_uint_ref", op.name),
            op,
            "&BigUint",
            "usize",
            "BigUint",
        ));
    } else {
        endpoints.push(BigNumOperatorTestEndpoint::new_bin(
            &format!("{}_big_uint", op.name),
            op,
            "BigUint",
            "BigUint",
            "BigUint",
        ));
        endpoints.push(BigNumOperatorTestEndpoint::new_bin(
            &format!("{}_big_uint_ref", op.name),
            op,
            "&BigUint",
            "&BigUint",
            "BigUint",
        ));
    }

    endpoints
}

fn write_endpoint(out: &mut String, endpoint: &BigNumOperatorTestEndpoint) {
    write!(
        out,
        "
    #[endpoint]
    fn {}(&self, a: {}, b: {}) -> {} {{{}}}",
        endpoint.fn_name, endpoint.a_type, endpoint.b_type, endpoint.return_type, endpoint.body
    )
    .unwrap();
}

fn section_comment(out: &mut String, comment: &str) {
    writeln!(out, "\n\n    // {}", comment).unwrap();
}

fn write_filtered_endpoints(
    endpoints: &[BigNumOperatorTestEndpoint],
    op_group: OpGroup,
    assign: bool,
    out: &mut String,
) {
    for endpoint in endpoints {
        if endpoint.op_info.group == op_group && endpoint.op_info.assign == assign {
            write_endpoint(out, endpoint);
        }
    }
}

/// Generates a Rust trait similar to BigIntOperators, with methods for each operator in OPS.
pub fn generate_big_int_operators_trait() -> String {
    let mut out = BIG_NUM_OPERATORS_PRELUDE.to_string();
    let ops = OperatorList::create();
    let endpoints = ops.0.iter().flat_map(create_endpoints).collect::<Vec<_>>();

    section_comment(&mut out, "Arithmetic binary operators");
    write_filtered_endpoints(&endpoints, OpGroup::Arithmetic, false, &mut out);

    section_comment(&mut out, "Arithmetic assign operators");
    write_filtered_endpoints(&endpoints, OpGroup::Arithmetic, true, &mut out);

    section_comment(&mut out, "Bitwise binary operators");
    write_filtered_endpoints(&endpoints, OpGroup::Bitwise, false, &mut out);

    section_comment(&mut out, "Bitwise assign operators");
    write_filtered_endpoints(&endpoints, OpGroup::Bitwise, true, &mut out);

    section_comment(&mut out, "Bitwise shift binary operators");
    write_filtered_endpoints(&endpoints, OpGroup::Shift, false, &mut out);

    section_comment(&mut out, "Bitwise shift assign operators");
    write_filtered_endpoints(&endpoints, OpGroup::Shift, true, &mut out);

    writeln!(&mut out, "\n}}").unwrap();

    out
}

const BIG_NUM_OPERATORS_PRELUDE: &str = r#"// Code generated by tools/op-test-gen crate. DO NOT EDIT.

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!!!!!!!!!!!!!!!!!!!!! AUTO-GENERATED FILE !!!!!!!!!!!!!!!!!!!!!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

multiversx_sc::imports!();

/// Checks that BigUint/BigInt operators work as expected.
#[multiversx_sc::module]
#[allow(clippy::redundant_clone)]
pub trait BigIntOperators {
    // Endpoints grouped into several sections:"#;
