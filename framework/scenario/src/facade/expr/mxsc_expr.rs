use multiversx_chain_scenario_format::{
    interpret_trait::InterpreterContext, value_interpreter::interpret_string,
};
use multiversx_sc::types::{AnnotatedValue, ManagedBuffer, TxCodeValue};

use crate::ScenarioTxEnv;

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

    fn to_value(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        let context = InterpreterContext::new()
            .with_dir(env.env_data().context_path.clone())
            .with_allowed_missing_files();
        let value = interpret_string(&format!("{MXSC_PREFIX}{}", self.0), &context);
        value.into()
    }
}

impl<'a, Env> TxCodeValue<Env> for MxscExpr<'a> where Env: ScenarioTxEnv {}

impl<'a> MxscExpr<'a> {
    pub fn eval_to_expr(&self) -> String {
        format!("{MXSC_PREFIX}{}", self.0)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::imports::MxscExpr;

    fn assert_eq_eval(expr: &'static str, expected: &str) {
        assert_eq!(&MxscExpr(expr).eval_to_expr(), expected);
    }

    #[test]
    fn test_address_value() {
        assert_eq_eval("output/adder.mxsc.json", "mxsc:output/adder.mxsc.json");
    }
}
