use crate::ScenarioTxEnv;

use multiversx_chain_scenario_format::{
    interpret_trait::InterpreterContext, value_interpreter::interpret_string,
};
use multiversx_sc::{
    api::ManagedTypeApi,
    types::{AnnotatedValue, BigUint, ManagedBuffer, TxEgldValue, TxGasValue},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumExpr<'a>(pub &'a str);

fn interpret_big_uint<Api>(s: &str) -> BigUint<Api>
where
    Api: ManagedTypeApi,
{
    let bytes = interpret_string(s, &InterpreterContext::new());
    BigUint::from_bytes_be(&bytes)
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for NumExpr<'_>
where
    Env: ScenarioTxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.0.into()
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        interpret_big_uint(self.0)
    }
}

impl<Env> AnnotatedValue<Env, u64> for NumExpr<'_>
where
    Env: ScenarioTxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.0.into()
    }

    fn to_value(&self, _env: &Env) -> u64 {
        interpret_big_uint::<Env::Api>(self.0).to_u64().unwrap()
    }
}

impl<Env> TxEgldValue<Env> for NumExpr<'_> where Env: ScenarioTxEnv {}
impl<Env> TxGasValue<Env> for NumExpr<'_> where Env: ScenarioTxEnv {}
