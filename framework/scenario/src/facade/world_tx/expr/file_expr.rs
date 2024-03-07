use core::ptr;
use std::path::PathBuf;

use multiversx_chain_scenario_format::{
    interpret_trait::InterpreterContext, value_interpreter::interpret_string,
};
use multiversx_sc::types::{
    AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv, TxFrom, TxFromSpecified, TxTo,
    TxToSpecified,
};

use crate::{api::StaticApi, ScenarioTxEnvironment};

const FILE_PREFIX: &str = "file:";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileExpr<'a>(pub &'a str);

impl<'a> AnnotatedValue<ScenarioTxEnvironment, ManagedBuffer<StaticApi>> for FileExpr<'a> {
    fn annotation(&self, _env: &ScenarioTxEnvironment) -> ManagedBuffer<StaticApi> {
        let mut result = ManagedBuffer::new_from_bytes(FILE_PREFIX.as_bytes());
        result.append_bytes(self.0.as_bytes());
        result
    }

    fn into_value(self, env: &ScenarioTxEnvironment) -> ManagedBuffer<StaticApi> {
        let context = InterpreterContext::new().with_dir(env.context_path.clone());
        let value = interpret_string(&format!("{FILE_PREFIX}{}", self.0), &context);
        value.into()
    }
}
