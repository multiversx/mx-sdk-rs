use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc::types::{
    AnnotatedValue, BigUint, Code, EsdtTokenIdentifier, ManagedAddress, ManagedBuffer, TxCodeValue,
    TxEnv, TxGas,
};

use crate::scenario_model::{AddressValue, BigUintValue, BytesKey, BytesValue, U64Value};

pub fn address_annotated<Env, Addr>(env: &Env, from: &Addr) -> AddressValue
where
    Env: TxEnv,
    Addr: AnnotatedValue<Env, ManagedAddress<Env::Api>>,
{
    let annotation = from.annotation(env).to_string();
    AddressValue {
        value: from.to_value(env).to_address(),
        original: ValueSubTree::Str(annotation),
    }
}

pub fn u64_annotated<Env, T>(env: &Env, from: &T) -> U64Value
where
    Env: TxEnv,
    T: AnnotatedValue<Env, u64>,
{
    let annotation = from.annotation(env).to_string();
    U64Value {
        value: from.to_value(env),
        original: ValueSubTree::Str(annotation),
    }
}

pub fn big_uint_annotated<Env, T>(env: &Env, from: &T) -> BigUintValue
where
    Env: TxEnv,
    T: AnnotatedValue<Env, BigUint<Env::Api>>,
{
    let annotation = from.annotation(env).to_string();
    BigUintValue {
        value: from.to_value(env).to_alloc(),
        original: ValueSubTree::Str(annotation),
    }
}

pub fn bytes_annotated<Env, T>(env: &Env, value: T) -> BytesValue
where
    Env: TxEnv,
    T: AnnotatedValue<Env, ManagedBuffer<Env::Api>>,
{
    let annotation = value.annotation(env).to_string();
    BytesValue {
        value: value.into_value(env).to_vec(),
        original: ValueSubTree::Str(annotation),
    }
}

pub fn token_identifier_annotated<Env, T>(env: &Env, value: T) -> BytesKey
where
    Env: TxEnv,
    T: AnnotatedValue<Env, EsdtTokenIdentifier<Env::Api>>,
{
    let annotation = value.annotation(env).to_string();
    BytesKey {
        value: value.into_value(env).into_managed_buffer().to_vec(),
        original: annotation,
    }
}

pub fn code_annotated<Env, CodeValue>(env: &Env, code: Code<CodeValue>) -> BytesValue
where
    Env: TxEnv,
    CodeValue: TxCodeValue<Env>,
{
    bytes_annotated(env, code.0)
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
