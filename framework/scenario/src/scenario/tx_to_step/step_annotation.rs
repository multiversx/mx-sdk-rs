use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc::types::{AnnotatedValue, Code, ManagedAddress, TxCodeValue, TxEnv, TxGas};

use crate::scenario_model::{AddressValue, BytesValue, U64Value};

pub fn address_annotated<Env, Addr>(env: &Env, from: Addr) -> AddressValue
where
    Env: TxEnv,
    Addr: AnnotatedValue<Env, ManagedAddress<Env::Api>>,
{
    let annotation = from.annotation(env).to_string();
    AddressValue {
        value: from.into_value(env).to_address(),
        original: ValueSubTree::Str(annotation),
    }
}

pub fn code_annotated<Env, CodeValue>(env: &Env, code: Code<CodeValue>) -> BytesValue
where
    Env: TxEnv,
    CodeValue: TxCodeValue<Env>,
{
    let annotation = code.0.annotation(env).to_string();
    BytesValue {
        value: code.0.into_value(env).to_vec(),
        original: ValueSubTree::Str(annotation),
    }
}

pub fn gas_annotated<Env, Gas>(env: &Env, gas: Gas) -> U64Value
where
    Env: TxEnv,
    Gas: TxGas<Env>,
{
    let annotation = gas.gas_annotation(env).to_string();
    U64Value {
        value: gas.gas_value(env),
        original: ValueSubTree::Str(annotation),
    }
}
