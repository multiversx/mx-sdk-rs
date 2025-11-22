use num_traits::identities::Zero;
use num_traits::ToPrimitive;

use multiversx_sc_scenario::imports::{
    Account, BytesValue, InterpretableFrom, ScQueryStep, Scenario, SetStateStep, TxExpect,
};
use multiversx_sc_scenario::scenario_format::interpret_trait::{InterpreterContext, IntoRaw};
use multiversx_sc_scenario::scenario_model::Step;

use crate::op_list::BaseOperator;
use crate::{create_all_endpoints, BigNumOperatorTestEndpoint, OpInfo, OperatorList};

const SC_ADDRESS_EXPR: &str = "sc:basic-features";

pub fn write_scenario() {
    let scenario = create_scenario();
    let scenario_raw = scenario.into_raw();
    let target_path =
        "../../contracts/feature-tests/basic-features/scenarios/big_num_ops.scen.json";
    scenario_raw.save_to_file(target_path);
    println!("Successfully rewrote {}", target_path);
}

pub fn create_scenario() -> Scenario {
    let mut scenario = Scenario::default();
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

    add_queries(&mut scenario);

    scenario
}

fn eval_op(a: &num_bigint::BigInt, b: &num_bigint::BigInt, op: &OpInfo) -> num_bigint::BigInt {
    println!("Evaluating: {} {} {}", a, op.symbol(), b);
    match op.base_operator {
        BaseOperator::Add => a + b,
        BaseOperator::Sub => a - b,
        BaseOperator::Mul => a * b,
        BaseOperator::Div => a / b,
        BaseOperator::Rem => a % b,
        BaseOperator::BitAnd => a & b,
        BaseOperator::BitOr => a | b,
        BaseOperator::BitXor => a ^ b,
        BaseOperator::Shl => {
            // For shifts, BigInt does not support shifting by BigInt directly.
            let shift_amount = b.to_usize().expect("Shift amount too large");
            a << shift_amount
        }
        BaseOperator::Shr => {
            let shift_amount = b.to_usize().expect("Shift amount too large");
            a >> shift_amount
        }
    }
}

fn add_query(
    scenario: &mut Scenario,
    endpoint: &BigNumOperatorTestEndpoint,
    a: num_bigint::BigInt,
    b: num_bigint::BigInt,
) {
    let tx_expect = if endpoint.op_info.base_operator.is_division() && b.is_zero() {
        TxExpect::err(10, "str:division by 0")
    } else {
        let result = eval_op(&a, &b, &endpoint.op_info);
        TxExpect::ok().result(&result.to_string())
    };

    scenario.steps.push(Step::ScQuery(
        ScQueryStep::new()
            .id(format!("{}({},{})", endpoint.fn_name, a, b))
            .to(SC_ADDRESS_EXPR)
            .function(&endpoint.fn_name)
            .argument(&a.to_string())
            .argument(&b.to_string())
            .expect(tx_expect),
    ));
}

fn add_queries(scenario: &mut Scenario) {
    let ops = OperatorList::create();
    let endpoints = create_all_endpoints(&ops);

    for endpoint in endpoints {
        add_query(
            scenario,
            &endpoint,
            num_bigint::BigInt::from(0),
            num_bigint::BigInt::from(0),
        );
    }
}
