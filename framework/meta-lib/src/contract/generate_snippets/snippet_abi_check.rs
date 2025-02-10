use std::fs::File;
use std::io::Write;

use multiversx_sc::abi::{ContractAbi, EndpointAbi, EndpointMutabilityAbi, InputAbi};
use serde::Deserialize;

use crate::abi_json::{serialize_abi_to_json, ContractAbiJson};

use super::snippet_crate_gen::LIB_SOURCE_FILE_NAME;
use super::snippet_sc_functions_gen::endpoint_args_when_called;
use super::snippet_type_map::map_abi_type_to_rust_type;
use crate::contract::generate_snippets::snippet_sc_functions_gen::DEFAULT_GAS;

#[derive(PartialEq, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ShortContractAbi {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub constructors: Vec<EndpointAbi>,
    #[serde(default, rename = "upgradeConstructor")]
    pub upgrade_constructors: Vec<EndpointAbi>,
    pub endpoints: Vec<EndpointAbi>,
}

impl From<ContractAbi> for ShortContractAbi {
    fn from(value: ContractAbi) -> Self {
        Self {
            name: value.name,
            constructors: value.constructors,
            upgrade_constructors: value.upgrade_constructors,
            endpoints: value.endpoints,
        }
    }
}

const PREV_ABI_NAME: &str = "prev-abi.json";

pub(crate) fn check_abi_differences(
    current_contract_abi: &ShortContractAbi,
    snippets_dir: &String,
    overwrite: bool,
) -> ShortContractAbi {
    if !overwrite {
        let prev_abi_path = format!("{}/prev-abi.json", snippets_dir);
        if let Ok(prev_abi_content) = std::fs::read_to_string(&prev_abi_path) {
            if let Ok(prev_abi) = serde_json::from_str::<ShortContractAbi>(&prev_abi_content) {
                let mut diff_abi = ShortContractAbi {
                    name: current_contract_abi.name.clone(),
                    constructors: vec![],
                    upgrade_constructors: vec![],
                    endpoints: vec![],
                };

                // changed and new constructors
                for constructor in &current_contract_abi.constructors {
                    if !prev_abi.constructors.contains(constructor) {
                        diff_abi.constructors.push(constructor.clone());
                    }
                }

                // changed and new upgrade constructors
                for upgrade_constructor in &current_contract_abi.upgrade_constructors {
                    if !prev_abi.upgrade_constructors.contains(upgrade_constructor) {
                        diff_abi
                            .upgrade_constructors
                            .push(upgrade_constructor.clone());
                    }
                }

                // changed and new endpoints
                for endpoint in &current_contract_abi.endpoints {
                    if !prev_abi.endpoints.contains(endpoint) {
                        diff_abi.endpoints.push(endpoint.clone());
                    }
                }

                // deleted endpoints
                for endpoint in &prev_abi.endpoints {
                    if !current_contract_abi.endpoints.contains(endpoint) {
                        diff_abi.endpoints.retain(|e| e.name != endpoint.name);
                    }
                }

                return diff_abi;
            }
        }
    }
    current_contract_abi.clone()
}

pub(crate) fn create_prev_abi_file(snippets_dir: &String, contract_abi: &ContractAbi) {
    let abi_json = ContractAbiJson::from(contract_abi);
    let abi_string = serialize_abi_to_json(&abi_json);

    let abi_file_path = format!("{snippets_dir}/{PREV_ABI_NAME}");
    let mut abi_file = File::create(abi_file_path).unwrap();
    write!(abi_file, "{abi_string}").unwrap();
}

pub(crate) fn add_new_endpoints_to_file(snippets_dir: &String, diff_abi: &ShortContractAbi) {
    let interact_lib_path = format!("{snippets_dir}/{LIB_SOURCE_FILE_NAME}");
    let file_content = std::fs::read_to_string(&interact_lib_path).unwrap();
    let mut updated_content = file_content.clone();

    for endpoint_abi in &diff_abi.endpoints {
        updated_content =
            insert_or_replace_function(&updated_content, endpoint_abi, &diff_abi.name);
    }

    for constructor in &diff_abi.constructors {
        updated_content = insert_or_replace_function(&updated_content, constructor, &diff_abi.name);
    }

    for upgrade_constructor in &diff_abi.upgrade_constructors {
        updated_content =
            insert_or_replace_function(&updated_content, upgrade_constructor, &diff_abi.name);
    }

    std::fs::write(interact_lib_path, updated_content).unwrap();
}

fn insert_or_replace_function(
    file_content: &str,
    endpoint_abi: &EndpointAbi,
    contract_name: &String,
) -> String {
    let function_signature = format!("pub async fn {}", endpoint_abi.rust_method_name);
    let mut updated_content = file_content.to_string();

    let new_function = {
        let mut function_buffer = String::new();
        write_endpoint_impl_to_string(&mut function_buffer, endpoint_abi, contract_name);
        function_buffer
    };

    if let Some(start) = file_content.find(&function_signature) {
        // remove existing function
        let mut balance = 0;
        let mut end = start;
        for (i, c) in file_content[start..].char_indices() {
            match c {
                '{' => balance += 1,
                '}' => {
                    balance -= 1;
                    if balance == 0 {
                        end = start + i + 1;
                        break;
                    }
                },
                _ => {},
            }
        }
        updated_content.replace_range(start..end, &new_function);
    } else {
        // append new function
        updated_content.push_str("\n\n");
        updated_content.push_str(&new_function);
    }

    updated_content
}

pub(crate) fn write_endpoint_impl_to_string(
    buffer: &mut String,
    endpoint_abi: &EndpointAbi,
    name: &String,
) {
    write_method_declaration_to_string(buffer, &endpoint_abi.rust_method_name);
    write_payments_declaration_to_string(buffer, &endpoint_abi.payable_in_tokens);
    write_endpoint_args_declaration_to_string(buffer, &endpoint_abi.inputs);

    if matches!(endpoint_abi.mutability, EndpointMutabilityAbi::Readonly) {
        write_contract_query_to_string(buffer, endpoint_abi, name);
    } else {
        write_contract_call_to_string(buffer, endpoint_abi, name);
    }

    buffer.push_str("    }\n");
    buffer.push('\n');
}

pub(crate) fn write_method_declaration_to_string(buffer: &mut String, endpoint_name: &str) {
    buffer.push_str(&format!("    pub async fn {endpoint_name}(&mut self) {{\n"));
}

pub(crate) fn write_payments_declaration_to_string(
    buffer: &mut String,
    accepted_tokens: &[String],
) {
    if accepted_tokens.is_empty() {
        return;
    }

    let biguint_default = map_abi_type_to_rust_type("BigUint".to_string());
    let first_accepted = &accepted_tokens[0];

    if first_accepted == "EGLD" {
        buffer.push_str(&format!(
            "        let egld_amount = {};\n",
            biguint_default.get_default_value_expr()
        ));
    } else {
        buffer.push_str(
            "        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = ",
        );
        buffer.push_str(&biguint_default.get_default_value_expr());
        buffer.push_str(";\n");
    }

    buffer.push('\n');
}

fn write_endpoint_args_declaration_to_string(buffer: &mut String, inputs: &[InputAbi]) {
    if inputs.is_empty() {
        return;
    }

    for input in inputs {
        let rust_type = map_abi_type_to_rust_type(input.type_names.abi.clone());
        buffer.push_str(&format!(
            "        let {} = {};\n",
            input.arg_name,
            rust_type.get_default_value_expr()
        ));
    }

    buffer.push('\n');
}

fn write_contract_call_to_string(buffer: &mut String, endpoint_abi: &EndpointAbi, name: &String) {
    let payment_snippet = if endpoint_abi.payable_in_tokens.is_empty() {
        "".to_string()
    } else if endpoint_abi.payable_in_tokens[0] == "EGLD" {
        "\n            .egld(egld_amount)".to_string()
    } else {
        "\n            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))".to_string()
    };

    buffer.push_str(&format!(
        r#"        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas({DEFAULT_GAS})
            .typed(proxy::{}Proxy)
            .{}({}){}
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {{response:?}}");
"#,
        name,
        endpoint_abi.rust_method_name,
        endpoint_args_when_called(endpoint_abi.inputs.as_slice()),
        payment_snippet,
    ));
}

fn write_contract_query_to_string(buffer: &mut String, endpoint_abi: &EndpointAbi, name: &String) {
    buffer.push_str(&format!(
        r#"        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::{}Proxy)
            .{}({})
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {{result_value:?}}");
"#,
        name,
        endpoint_abi.rust_method_name,
        endpoint_args_when_called(endpoint_abi.inputs.as_slice()),
    ));
}
