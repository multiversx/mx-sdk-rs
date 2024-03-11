use core::ptr;
use std::path::PathBuf;

use multiversx_chain_scenario_format::{
    interpret_trait::InterpreterContext, value_interpreter::interpret_string,
};
use multiversx_sc::types::{
    AnnotatedValue, ManagedAddress, ManagedBuffer, TxCodeValue, TxEnv, TxFrom, TxFromSpecified,
    TxTo, TxToSpecified,
};

use crate::{api::StaticApi, ScenarioEnvExec, ScenarioTxEnv, ScenarioTxEnvData};

const MXSC_PREFIX: &str = "mxsc:";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MxscExpr<'a>(pub &'a str);

impl<'a, Env> AnnotatedValue<Env, ManagedBuffer<Env::Api>> for MxscExpr<'a>
where
    Env: ScenarioTxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        let mut result = ManagedBuffer::new_from_bytes(MXSC_PREFIX.as_bytes());
        result.append_bytes(self.0.as_bytes());
        result
    }

    fn into_value(self, env: &Env) -> ManagedBuffer<Env::Api> {
        let context = InterpreterContext::new()
            .with_dir(env.env_data().context_path.clone())
            .with_allowed_missing_files();
        let value = interpret_string(&format!("{MXSC_PREFIX}{}", self.0), &context);
        value.into()
    }
}

impl<'a, Env> TxCodeValue<Env> for MxscExpr<'a> where Env: ScenarioTxEnv {}
