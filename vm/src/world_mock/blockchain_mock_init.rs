use crate::scenario_format::{
    interpret_trait::InterpreterContext, value_interpreter::interpret_string,
};
use multiversx_sc::contract_base::{CallableContractBuilder, ContractAbiProvider};

use crate::DebugApi;

use super::{BlockchainMock, ContractContainer};

impl BlockchainMock {
    pub fn interpreter_context(&self) -> InterpreterContext {
        InterpreterContext::new(self.current_dir.clone())
    }

    pub fn register_contract_container(
        &mut self,
        expression: &str,
        contract_container: ContractContainer,
    ) {
        let contract_bytes = interpret_string(expression, &self.interpreter_context());
        self.contract_map
            .register_contract(contract_bytes, contract_container);
    }

    /// Links a contract path in a test to a contract implementation.
    pub fn register_contract<B: CallableContractBuilder>(
        &mut self,
        expression: &str,
        contract_builder: B,
    ) {
        self.register_contract_container(
            expression,
            ContractContainer::new(contract_builder.new_contract_obj::<DebugApi>(), None, false),
        )
    }

    /// Links a contract path in a test to a multi-contract output.
    ///
    /// This simulates the effects of building such a contract with only part of the endpoints.
    pub fn register_partial_contract<Abi, B>(
        &mut self,
        expression: &str,
        contract_builder: B,
        sub_contract_name: &str,
    ) where
        Abi: ContractAbiProvider,
        B: CallableContractBuilder,
    {
        let multi_contract_config = multiversx_sc_meta::multi_contract_config::<Abi>(
            self.current_dir
                .join("multicontract.toml")
                .to_str()
                .unwrap(),
        );
        let sub_contract = multi_contract_config.find_contract(sub_contract_name);
        let contract_obj = if sub_contract.settings.external_view {
            contract_builder.new_contract_obj::<multiversx_sc::api::ExternalViewApi<DebugApi>>()
        } else {
            contract_builder.new_contract_obj::<DebugApi>()
        };

        self.register_contract_container(
            expression,
            ContractContainer::new(
                contract_obj,
                Some(sub_contract.all_exported_function_names()),
                sub_contract.settings.panic_message,
            ),
        );
    }
}
