use crate::{
    api::DebugApi,
    executor::debug::ContractContainer,
    multiversx_sc::{
        api,
        contract_base::{CallableContractBuilder, ContractAbiProvider},
    },
    scenario_format::interpret_trait::InterpreterContext,
    scenario_model::BytesValue,
    ScenarioWorld,
};
use multiversx_chain_scenario_format::interpret_trait::InterpretableFrom;

use multiversx_sc_meta_lib::contract::sc_config::ContractVariant;

use super::expr::RegisterCodeSource;

impl ScenarioWorld {
    pub fn interpreter_context(&self) -> InterpreterContext {
        InterpreterContext::default()
            .with_dir(self.current_dir.clone())
            .with_allowed_missing_files()
    }

    /// Convenient way of creating a code expression based on the current context
    /// (i.e. with the paths resolved, as configured).
    pub fn code_expression(&self, path: &str) -> BytesValue {
        BytesValue::interpret_from(path, &self.interpreter_context())
    }

    pub fn register_contract_container(
        &mut self,
        expression: impl RegisterCodeSource,
        contract_container: ContractContainer,
    ) {
        let contract_bytes = expression.into_code(self.new_env_data());
        self.get_mut_debugger_backend()
            .vm_runner
            .contract_map_ref
            .lock()
            .register_contract(contract_bytes, contract_container);
    }

    /// Links a contract path in a test to a contract implementation.
    pub fn register_contract<B: CallableContractBuilder>(
        &mut self,
        expression: impl RegisterCodeSource,
        contract_builder: B,
    ) {
        self.register_contract_container(
            expression,
            ContractContainer::new(contract_builder.new_contract_obj::<DebugApi>(), None, false),
        )
    }

    #[deprecated(
        since = "0.37.0",
        note = "Got renamed to `register_contract`, but not completely removed, in order to ease test migration. Please replace with `register_contract`."
    )]
    pub fn register_contract_builder<B: CallableContractBuilder>(
        &mut self,
        expression: &str,
        contract_builder: B,
    ) {
        self.register_contract(expression, contract_builder)
    }

    /// Links a contract path in a test to a multi-contract output.
    ///
    /// This simulates the effects of building such a contract with only part of the endpoints.
    pub fn register_partial_contract<Abi, B>(
        &mut self,
        expression: impl RegisterCodeSource,
        contract_builder: B,
        sub_contract_name: &str,
    ) where
        Abi: ContractAbiProvider,
        B: CallableContractBuilder,
    {
        let multi_contract_config =
            multiversx_sc_meta_lib::multi_contract_config::<Abi>(self.current_dir.as_path());
        let contract_variant = multi_contract_config.find_contract(sub_contract_name);
        self.register_contract_variant(expression, contract_builder, contract_variant);
    }

    /// Links a contract path in a test to a multi-contract output.
    ///
    /// This simulates the effects of building such a contract with only part of the endpoints.
    pub fn register_contract_variant<B>(
        &mut self,
        expression: impl RegisterCodeSource,
        contract_builder: B,
        contract_variant: &ContractVariant,
    ) where
        B: CallableContractBuilder,
    {
        let contract_obj = if contract_variant.settings.external_view {
            contract_builder.new_contract_obj::<api::ExternalViewApi<DebugApi>>()
        } else {
            contract_builder.new_contract_obj::<DebugApi>()
        };

        self.register_contract_container(
            expression,
            ContractContainer::new(
                contract_obj,
                Some(contract_variant.all_exported_function_names()),
                contract_variant.settings.panic_message,
            ),
        );
    }
}
