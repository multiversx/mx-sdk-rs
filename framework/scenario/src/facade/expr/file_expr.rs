use multiversx_chain_scenario_format::{
    interpret_trait::InterpreterContext, value_interpreter::interpret_string,
};
use multiversx_sc::types::{AnnotatedValue, ManagedBuffer, TxCodeValue};

use crate::{ScenarioTxEnv, ScenarioTxEnvData};

use super::RegisterCodeSource;

const FILE_PREFIX: &str = "file:";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileExpr<'a>(pub &'a str);

impl<'a> FileExpr<'a> {
    pub fn eval_to_expr(&self) -> String {
        format!("{FILE_PREFIX}{}", self.0)
    }

    pub fn resolve_contents(&self, context: &InterpreterContext) -> Vec<u8> {
        interpret_string(&format!("{FILE_PREFIX}{}", self.0), context)
    }
}

impl<'a, Env> AnnotatedValue<Env, ManagedBuffer<Env::Api>> for FileExpr<'a>
where
    Env: ScenarioTxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.eval_to_expr().into()
    }

    fn to_value(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        self.resolve_contents(&env.env_data().interpreter_context())
            .into()
    }
}

impl<'a, Env> TxCodeValue<Env> for FileExpr<'a> where Env: ScenarioTxEnv {}

impl<'a> RegisterCodeSource for FileExpr<'a> {
    fn into_code(self, env_data: ScenarioTxEnvData) -> Vec<u8> {
        self.resolve_contents(&env_data.interpreter_context())
    }
}
