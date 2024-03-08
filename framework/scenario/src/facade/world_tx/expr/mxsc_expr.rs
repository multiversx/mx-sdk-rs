use core::ptr;
use std::path::PathBuf;

use multiversx_chain_scenario_format::{
    interpret_trait::InterpreterContext, value_interpreter::interpret_string,
};
use multiversx_sc::types::{
    AnnotatedValue, ManagedAddress, ManagedBuffer, TxCodeValue, TxEnv, TxFrom, TxFromSpecified,
    TxTo, TxToSpecified,
};

use crate::{api::StaticApi, ScenarioTxEnv};

const MXSC_PREFIX: &str = "mxsc:";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MxscExpr<'a>(pub &'a str);

impl<'a> AnnotatedValue<ScenarioTxEnv, ManagedBuffer<StaticApi>> for MxscExpr<'a> {
    fn annotation(&self, _env: &ScenarioTxEnv) -> ManagedBuffer<StaticApi> {
        let mut result = ManagedBuffer::new_from_bytes(MXSC_PREFIX.as_bytes());
        result.append_bytes(self.0.as_bytes());
        result
    }

    fn into_value(self, env: &ScenarioTxEnv) -> ManagedBuffer<StaticApi> {
        let context = InterpreterContext::new()
            .with_dir(env.context_path.clone())
            .with_allowed_missing_files();
        let value = interpret_string(&format!("{MXSC_PREFIX}{}", self.0), &context);
        value.into()
    }
}

impl<'a> TxCodeValue<ScenarioTxEnv> for MxscExpr<'a> {}
