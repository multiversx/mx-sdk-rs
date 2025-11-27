use num_traits::identities::Zero;
use num_traits::{Signed, ToPrimitive};

use multiversx_sc_scenario::imports::{
    Account, BytesValue, InterpretableFrom, ScQueryStep, Scenario, SetStateStep, TxExpect,
};
use multiversx_sc_scenario::scenario_format::interpret_trait::{InterpreterContext, IntoRaw};
use multiversx_sc_scenario::scenario_model::Step;

use crate::op_list::BaseOperator;
use crate::{create_all_endpoints, BigNumOperatorTestEndpoint, OperatorList, ValueType};

const SC_ADDRESS_EXPR: &str = "sc:basic-features";

pub fn write_scenarios() {
    write_scenario_arith(
        "../../contracts/feature-tests/basic-features/scenarios/big_num_ops_arith.scen.json",
    );
    write_scenario_bitwise(
        "../../contracts/feature-tests/basic-features/scenarios/big_num_ops_bitwise.scen.json",
    );
    write_scenario_shift(
        "../../contracts/feature-tests/basic-features/scenarios/big_num_ops_shift.scen.json",
    );
}

pub fn write_scenario_arith(target_path: &str) {
    let mut scenario = create_scenario();
    let ops = OperatorList::create();
    let endpoints = create_all_endpoints(&ops);

    let numbers = vec![
        num_bigint::BigInt::from(0),
        num_bigint::BigInt::from(1),
        num_bigint::BigInt::from(255),
        num_bigint::BigInt::from(18446744073709551615i128),
        num_bigint::BigInt::from(18446744073709551616i128),
        num_bigint::BigInt::from(-1),
        num_bigint::BigInt::from(-256),
    ];

    for endpoint in endpoints {
        for a in &numbers {
            for b in &numbers {
                if let Some(tx_expect) = eval_op_arith(a, b, &endpoint) {
                    add_query(&mut scenario, &endpoint, a, b, tx_expect);
                }
            }
        }
    }

    save_scenario(scenario, target_path);
}

fn eval_op_arith(
    a: &num_bigint::BigInt,
    b: &num_bigint::BigInt,
    endpoint: &BigNumOperatorTestEndpoint,
) -> Option<TxExpect> {
    if !endpoint.a_type.is_signed() && a.is_negative() {
        return None;
    }
    if !endpoint.b_type.is_signed() && b.is_negative() {
        return None;
    }
    if endpoint.a_type.is_non_zero() && a.is_zero() {
        return None;
    }
    if endpoint.b_type.is_non_zero() && b.is_zero() {
        return None;
    }
    if endpoint.b_type == ValueType::U32 && b > &num_bigint::BigInt::from(u32::MAX) {
        return None;
    }
    if endpoint.b_type == ValueType::U64 && b > &num_bigint::BigInt::from(i64::MAX) {
        // conversion to i64 is needed, as BigInt does not support u64 directly
        // anything above i64::MAX is not supported
        return None;
    }

    match endpoint.op_info.base_operator {
        BaseOperator::Add => tx_expect_ok(endpoint, a + b),
        BaseOperator::Sub => {
            let result = a - b;
            if !endpoint.return_type.is_signed() && result.is_negative() {
                return Some(TxExpect::err(
                    4,
                    "str:cannot subtract because result would be negative",
                ));
            }
            if endpoint.return_type.is_non_zero() && result.is_zero() {
                return Some(TxExpect::err(4, "str:zero value not allowed"));
            }
            tx_expect_ok(endpoint, result)
        }
        BaseOperator::Mul => {
            let result = a * b;
            if endpoint.return_type.is_non_zero() && result.is_zero() {
                return Some(TxExpect::err(4, "str:zero value not allowed"));
            }
            tx_expect_ok(endpoint, result)
        }
        BaseOperator::Div => {
            if b.is_zero() {
                return Some(TxExpect::err(10, "str:division by 0"));
            }

            let result = a / b;
            if endpoint.return_type.is_non_zero() && result.is_zero() {
                return Some(TxExpect::err(4, "str:zero value not allowed"));
            }
            tx_expect_ok(endpoint, result)
        }
        BaseOperator::Rem => {
            if b.is_zero() {
                return Some(TxExpect::err(10, "str:division by 0"));
            }

            let result = a % b;
            if endpoint.return_type.is_non_zero() && result.is_zero() {
                return Some(TxExpect::err(4, "str:zero value not allowed"));
            }
            tx_expect_ok(endpoint, result)
        }
        _ => None,
    }
}

pub fn write_scenario_bitwise(target_path: &str) {
    let mut scenario = create_scenario();
    let ops = OperatorList::create();
    let endpoints = create_all_endpoints(&ops);

    let numbers = vec![
        num_bigint::BigInt::from(0),
        num_bigint::BigInt::from(1),
        num_bigint::BigInt::from(2),
        num_bigint::BigInt::from(255),
        num_bigint::BigInt::from(256),
        num_bigint::BigInt::from(18446744073709551615i128),
        num_bigint::BigInt::from(18446744073709551616i128),
    ];

    for endpoint in endpoints {
        for a in &numbers {
            for b in &numbers {
                if let Some(tx_expect) = eval_op_bitwise(a, b, &endpoint) {
                    add_query(&mut scenario, &endpoint, a, b, tx_expect);
                }
            }
        }
    }

    save_scenario(scenario, target_path);
}

fn eval_op_bitwise(
    a: &num_bigint::BigInt,
    b: &num_bigint::BigInt,
    endpoint: &BigNumOperatorTestEndpoint,
) -> Option<TxExpect> {
    assert!(
        !a.is_negative() && !b.is_negative(),
        "Bitwise ops only for non-negative numbers"
    );
    if endpoint.b_type == ValueType::U32 && b > &num_bigint::BigInt::from(u32::MAX) {
        return None;
    }
    if endpoint.b_type == ValueType::U64 && b > &num_bigint::BigInt::from(i64::MAX) {
        // conversion to i64 is needed, as BigInt does not support u64 directly
        // anything above i64::MAX is not supported
        return None;
    }

    match endpoint.op_info.base_operator {
        BaseOperator::BitAnd => tx_expect_ok(endpoint, a & b),
        BaseOperator::BitOr => tx_expect_ok(endpoint, a | b),
        BaseOperator::BitXor => tx_expect_ok(endpoint, a ^ b),
        _ => None,
    }
}

pub fn write_scenario_shift(target_path: &str) {
    let mut scenario = create_scenario();
    let ops = OperatorList::create();
    let endpoints = create_all_endpoints(&ops);

    let numbers = vec![
        num_bigint::BigInt::from(0),
        num_bigint::BigInt::from(1),
        num_bigint::BigInt::from(18446744073709551615i128),
        num_bigint::BigInt::from(18446744073709551616i128),
    ];

    let shift_amounts = vec![
        num_bigint::BigInt::from(0),
        num_bigint::BigInt::from(1),
        num_bigint::BigInt::from(1000),
    ];

    for endpoint in endpoints {
        for a in &numbers {
            for b in &shift_amounts {
                if let Some(tx_expect) = eval_op_shift(a, b, &endpoint) {
                    add_query(&mut scenario, &endpoint, a, b, tx_expect);
                }
            }
        }
    }

    save_scenario(scenario, target_path);
}

fn eval_op_shift(
    a: &num_bigint::BigInt,
    b: &num_bigint::BigInt,
    endpoint: &BigNumOperatorTestEndpoint,
) -> Option<TxExpect> {
    assert!(
        !a.is_negative() && !b.is_negative(),
        "Shift ops only for non-negative numbers"
    );

    match endpoint.op_info.base_operator {
        BaseOperator::Shl => {
            // For shifts, BigInt does not support shifting by BigInt directly.
            let shift_amount = b.to_usize()?;
            tx_expect_ok(endpoint, a << shift_amount)
        }
        BaseOperator::Shr => {
            let shift_amount = b.to_usize()?;
            tx_expect_ok(endpoint, a >> shift_amount)
        }
        _ => None,
    }
}

pub fn create_scenario() -> Scenario {
    let mut scenario = Scenario::default()
        .with_comment("Code generated by mx-sdk-rs/tools/op-test-gen. DO NOT EDIT.");
    let interpreter_context = InterpreterContext::new().with_allowed_missing_files();

    scenario.steps.push(Step::SetState(
        SetStateStep::new().put_account(
            SC_ADDRESS_EXPR,
            Account::default()
                .nonce("0")
                .balance("0")
                .code(BytesValue::interpret_from(
                    "mxsc:../output/basic-features.mxsc.json",
                    &interpreter_context,
                )),
        ),
    ));

    scenario
}

fn save_scenario(scenario: Scenario, target_path: &str) {
    let scenario_raw = scenario.into_raw();
    scenario_raw.save_to_file(target_path);
    println!("Successfully rewrote {}", target_path);
}

fn tx_expect_ok(
    endpoint: &BigNumOperatorTestEndpoint,
    result: num_bigint::BigInt,
) -> Option<TxExpect> {
    Some(TxExpect::ok().result(&serialize_arg(endpoint.return_type, &result)))
}

fn serialize_arg(arg_type: ValueType, n: &num_bigint::BigInt) -> String {
    if arg_type.is_signed() && n.is_positive() {
        format!("+{n}")
    } else {
        n.to_string()
    }
}

fn add_query(
    scenario: &mut Scenario,
    endpoint: &BigNumOperatorTestEndpoint,
    a: &num_bigint::BigInt,
    b: &num_bigint::BigInt,
    tx_expect: TxExpect,
) {
    scenario.steps.push(Step::ScQuery(
        ScQueryStep::new()
            .id(format!("{}({},{})", endpoint.fn_name, a, b))
            .to(SC_ADDRESS_EXPR)
            .function(&endpoint.fn_name)
            .argument(&serialize_arg(endpoint.a_type, a))
            .argument(&serialize_arg(endpoint.b_type, b))
            .expect(tx_expect),
    ));
}
