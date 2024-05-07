use multiversx_chain_scenario_format::value_interpreter::interpret_string;

use crate::ScenarioTxEnvData;

/// Used when registering a contract for debugging.
///
/// Any type that implements this trait can be passed to the `register_contract` method, and to its variants.
pub trait RegisterCodeSource {
    fn into_code(self, env_data: ScenarioTxEnvData) -> Vec<u8>;
}

impl RegisterCodeSource for &str {
    fn into_code(self, env_data: ScenarioTxEnvData) -> Vec<u8> {
        interpret_string(self, &env_data.interpreter_context())
    }
}

impl RegisterCodeSource for String {
    fn into_code(self, env_data: ScenarioTxEnvData) -> Vec<u8> {
        self.as_str().into_code(env_data)
    }
}

impl RegisterCodeSource for &String {
    fn into_code(self, env_data: ScenarioTxEnvData) -> Vec<u8> {
        self.as_str().into_code(env_data)
    }
}
