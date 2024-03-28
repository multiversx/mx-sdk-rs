use crate::{api::StaticApi, ScenarioEnvExec, ScenarioTxEnv, ScenarioTxEnvData};
use core::ptr;
use multiversx_chain_scenario_format::{
    interpret_trait::InterpreterContext, value_interpreter::interpret_string,
};
use multiversx_sc::{
    api::ManagedTypeApi,
    types::{
        AnnotatedValue, BigUint, ManagedAddress, ManagedBuffer, TxCodeValue, TxEgldValue, TxEnv,
        TxFrom, TxFromSpecified, TxGasValue, TxTo, TxToSpecified,
    },
};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumExpr<'a>(pub &'a str);

fn interpret_big_uint<Api>(s: &str) -> BigUint<Api>
where
    Api: ManagedTypeApi,
{
    let bytes = interpret_string(s, &InterpreterContext::new());
    BigUint::from_bytes_be(&bytes)
}

impl<'a, Env> AnnotatedValue<Env, BigUint<Env::Api>> for NumExpr<'a>
where
    Env: ScenarioTxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.0.into()
    }

    fn to_value(&self, env: &Env) -> BigUint<Env::Api> {
        interpret_big_uint(self.0)
    }
}

impl<'a, Env> AnnotatedValue<Env, u64> for NumExpr<'a>
where
    Env: ScenarioTxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.0.into()
    }

    fn to_value(&self, env: &Env) -> u64 {
        interpret_big_uint::<Env::Api>(self.0).to_u64().unwrap()
    }
}

impl<'a, Env> TxEgldValue<Env> for NumExpr<'a> where Env: ScenarioTxEnv {}
impl<'a, Env> TxGasValue<Env> for NumExpr<'a> where Env: ScenarioTxEnv {}
