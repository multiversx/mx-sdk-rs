use std::fs::File;
use std::io::Write;
use std::path::Path;

use multiversx_sc::abi::{ContractAbi, EndpointAbi, InputAbi, OutputAbi};
use serde::de::DeserializeOwned;
use serde::{de, Deserialize, Deserializer};

use crate::abi_json::{serialize_abi_to_json, ContractAbiJson};

use super::snippet_crate_gen::LIB_SOURCE_FILE_NAME;
use super::snippet_type_map::map_abi_type_to_rust_type;
use crate::contract::generate_snippets::snippet_sc_functions_gen::DEFAULT_GAS;

const PREV_ABI_NAME: &str = "prev-abi.json";

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ShortContractAbi {
    #[serde(default)]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_single_or_vec")]
    pub constructor: Vec<ShortEndpointAbi>,
    #[serde(
        default,
        rename = "upgradeConstructor",
        deserialize_with = "deserialize_single_or_vec"
    )]
    pub upgrade_constructor: Vec<ShortEndpointAbi>,
    #[serde(default)]
    pub endpoints: Vec<ShortEndpointAbi>,
    #[serde(skip_deserializing)]
    pub deleted_endpoints: Vec<ShortEndpointAbi>,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
pub(crate) struct ShortEndpointAbi {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub mutability: String,
    #[serde(default, skip_deserializing)]
    pub rust_method_name: String,
    #[serde(default)]
    pub payable_in_tokens: Vec<String>,
    #[serde(default)]
    pub inputs: Vec<ShortInputAbi>,
    #[serde(default)]
    pub outputs: Vec<ShortOutputAbi>,
    #[serde(default)]
    pub allow_multiple_var_args: bool,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
pub(crate) struct ShortInputAbi {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    pub multi_arg: bool,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
pub(crate) struct ShortOutputAbi {
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    pub multi_result: bool,
}

fn deserialize_single_or_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Array(vec) => {
            serde_json::from_value(serde_json::Value::Array(vec)).map_err(de::Error::custom)
        },
        _ => Ok(serde_json::from_value(value.clone())
            .map(|single| vec![single])
            .unwrap_or_else(|_| panic!("error at {value:?}"))),
    }
}

impl From<EndpointAbi> for ShortEndpointAbi {
    fn from(value: EndpointAbi) -> Self {
        Self {
            name: value.name,
            mutability: format!("{:?}", value.mutability).to_lowercase(),
            rust_method_name: value.rust_method_name,
            payable_in_tokens: value.payable_in_tokens,
            inputs: value.inputs.into_iter().map(ShortInputAbi::from).collect(),
            outputs: value
                .outputs
                .into_iter()
                .map(ShortOutputAbi::from)
                .collect(),
            allow_multiple_var_args: value.allow_multiple_var_args,
        }
    }
}

impl From<InputAbi> for ShortInputAbi {
    fn from(value: InputAbi) -> Self {
        Self {
            name: value.arg_name,
            type_name: value.type_names.abi,
            multi_arg: value.multi_arg,
        }
    }
}

impl From<OutputAbi> for ShortOutputAbi {
    fn from(value: OutputAbi) -> Self {
        Self {
            type_name: value.type_names.abi,
            multi_result: value.multi_result,
        }
    }
}

impl From<ContractAbi> for ShortContractAbi {
    fn from(value: ContractAbi) -> Self {
        Self {
            name: value.name,
            constructor: value
                .constructors
                .into_iter()
                .map(ShortEndpointAbi::from)
                .collect(),
            upgrade_constructor: value
                .upgrade_constructors
                .into_iter()
                .map(ShortEndpointAbi::from)
                .collect(),
            endpoints: value
                .endpoints
                .into_iter()
                .map(ShortEndpointAbi::from)
                .collect(),
            deleted_endpoints: vec![],
        }
    }
}

pub(crate) fn check_abi_differences(
    current_contract_abi: &ShortContractAbi,
    snippets_dir: &Path,
    overwrite: bool,
) -> ShortContractAbi {
    if !overwrite {
        let prev_abi_path = snippets_dir.join(PREV_ABI_NAME);

        if let Ok(prev_abi_content) = std::fs::read_to_string(&prev_abi_path) {
            if let Ok(mut prev_abi) = serde_json::from_str::<ShortContractAbi>(&prev_abi_content) {
                if prev_abi.constructor[0].name.is_empty() {
                    prev_abi.constructor[0].name = "init".to_string();
                    prev_abi.constructor[0].mutability = "mutable".to_string();
                    prev_abi.constructor[0].rust_method_name = "init".to_string();
                }

                if prev_abi.upgrade_constructor[0].name.is_empty() {
                    prev_abi.upgrade_constructor[0].name = "upgrade".to_string();
                    prev_abi.upgrade_constructor[0].mutability = "mutable".to_string();
                    prev_abi.upgrade_constructor[0].rust_method_name = "upgrade".to_string();
                }

                println!("prev ABI constructor {:?}", prev_abi.constructor);
                println!(
                    "prev ABI upgrade constructor {:?}",
                    prev_abi.upgrade_constructor
                );
                println!("prev ABI endpdoints {:?}", prev_abi.endpoints);
                println!("**********");
                println!(
                    "CURRENT ABI constructor {:?}",
                    current_contract_abi.constructor
                );
                println!(
                    "CURRENT ABI upgrade constructor {:?}",
                    current_contract_abi.upgrade_constructor
                );
                println!("CURRENT ABI endpoints {:?}", current_contract_abi.endpoints);

                let mut diff_abi = ShortContractAbi {
                    name: current_contract_abi.name.clone(),
                    constructor: vec![],
                    upgrade_constructor: vec![],
                    endpoints: vec![],
                    deleted_endpoints: vec![],
                };

                // changed and new constructors
                for constructor in &current_contract_abi.constructor {
                    if !prev_abi
                        .constructor
                        .iter()
                        .any(|e| e.name == constructor.name)
                    {
                        diff_abi.constructor.push(constructor.clone());
                    }
                }

                // changed and new upgrade constructors
                // PartialEq doesn't work
                for upgrade_constructor in &current_contract_abi.upgrade_constructor {
                    if !prev_abi
                        .upgrade_constructor
                        .iter()
                        .any(|e| e.name == upgrade_constructor.name)
                    {
                        diff_abi
                            .upgrade_constructor
                            .push(upgrade_constructor.clone());
                    }
                }

                // changed and new endpoints
                // RUST_METHOD_NAME not inherited from EndpointAbi
                for endpoint in &current_contract_abi.endpoints {
                    if !prev_abi.endpoints.iter().any(|e| e.name == endpoint.name) {
                        diff_abi.endpoints.push(endpoint.clone());
                    }
                }

                // deleted endpoints
                for endpoint in &prev_abi.endpoints {
                    if !current_contract_abi
                        .endpoints
                        .iter()
                        .any(|e| e.name == endpoint.name)
                    {
                        diff_abi.deleted_endpoints.push(endpoint.clone());
                    }
                }

                // deleted endpoints arrive in struct without a rust name
                println!("diff_abi endpoints {:?}", diff_abi.endpoints);
                println!(
                    "diff_abi deleted endpoints {:?}",
                    diff_abi.deleted_endpoints
                );

                return diff_abi;
            }
        }
    }
    current_contract_abi.clone()
}

pub(crate) fn create_prev_abi_file(snippets_dir: &Path, contract_abi: &ContractAbi) {
    let abi_json = ContractAbiJson::from(contract_abi);
    let abi_string = serialize_abi_to_json(&abi_json);
    let abi_file_path = snippets_dir.join(PREV_ABI_NAME);

    let mut abi_file = File::create(abi_file_path).unwrap();
    write!(abi_file, "{abi_string}").unwrap();
}

pub(crate) fn add_new_endpoints_to_file(snippets_dir: &Path, diff_abi: &ShortContractAbi) {
    let interact_lib_path = snippets_dir.join("src").join(LIB_SOURCE_FILE_NAME);
    let file_content = std::fs::read_to_string(&interact_lib_path).unwrap();
    let mut updated_content = file_content.clone();

    for deleted_endpoint in &diff_abi.deleted_endpoints {
        remove_function2(snippets_dir, deleted_endpoint);
    }

    for endpoint_abi in &diff_abi.endpoints {
        updated_content = insert_function(&updated_content, endpoint_abi, &diff_abi.name);
    }

    for constructor in &diff_abi.constructor {
        updated_content = insert_function(&updated_content, constructor, &diff_abi.name);
    }

    for upgrade_constructor in &diff_abi.upgrade_constructor {
        updated_content = insert_function(&updated_content, upgrade_constructor, &diff_abi.name);
    }

    std::fs::write(interact_lib_path, updated_content).unwrap();
}

fn insert_function(
    file_content: &str,
    endpoint_abi: &ShortEndpointAbi,
    contract_name: &String,
) -> String {
    let mut updated_content = file_content.to_string();

    println!("Inserting endpoint with name {:?}", endpoint_abi.name);

    let new_function = {
        let mut function_buffer = String::new();
        write_endpoint_impl_to_string(&mut function_buffer, endpoint_abi, contract_name);
        function_buffer
    };

    updated_content.push_str(&new_function);
    updated_content.push('\n');

    updated_content
}

pub(crate) fn remove_function2(snippets_dir: &Path, endpoint: &ShortEndpointAbi) {
    let interact_lib_path = snippets_dir.join("src").join(LIB_SOURCE_FILE_NAME);

    let file_content = std::fs::read_to_string(&interact_lib_path).unwrap();

    // find and remove the function
    let updated_content = find_and_remove_function(file_content, &endpoint.name);

    // delete the initial file
    std::fs::remove_file(&interact_lib_path).unwrap();

    // create a new file at the same path with the updated content
    std::fs::write(&interact_lib_path, &updated_content).unwrap();
}

pub(crate) fn find_and_remove_function(file_content: String, endpoint_name: &str) -> String {
    let lines: Vec<&str> = file_content.lines().collect();

    // find the start of the function
    if let Some(start_index) = lines.iter().position(|line| {
        !line.starts_with("//")
            && !line.starts_with("/*")
            && !line.ends_with("*/")
            && line.contains(&format!("pub async fn {}", endpoint_name))
    }) {
        // find the end of the function by tracking brace balance
        let mut balance = 0;
        let mut end_index = start_index;

        for (i, line) in lines.iter().enumerate().skip(start_index) {
            // count opening and closing braces
            balance += line.chars().filter(|&c| c == '{').count() as i32;
            balance -= line.chars().filter(|&c| c == '}').count() as i32;

            if balance == 0 {
                end_index = i;
                break;
            }
        }

        // remove the function block including whitespace
        let updated_lines: Vec<&str> = lines
            .iter()
            .enumerate()
            .filter(|&(i, _)| i < start_index || i > end_index)
            .map(|(_, &line)| line)
            .collect();

        // join the lines back together
        let updated_content = updated_lines.join("\n");

        println!("********");

        println!("updated content after deletion {updated_content:?}");

        updated_content
    } else {
        println!(
            "Cannot delete function {:?} from the file. Not found",
            endpoint_name
        );

        file_content
    }
}

pub(crate) fn write_endpoint_impl_to_string(
    buffer: &mut String,
    endpoint_abi: &ShortEndpointAbi,
    name: &String,
) {
    println!(
        "Writing endpoint impl for function with name {:?}",
        endpoint_abi.rust_method_name
    );
    write_method_declaration_to_string(buffer, &endpoint_abi.rust_method_name);
    write_payments_declaration_to_string(buffer, &endpoint_abi.payable_in_tokens);
    write_endpoint_args_declaration_to_string(buffer, &endpoint_abi.inputs);

    if endpoint_abi.mutability == *"readonly" {
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
        buffer.push_str(biguint_default.get_default_value_expr());
        buffer.push_str(";\n");
    }

    buffer.push('\n');
}

fn write_endpoint_args_declaration_to_string(buffer: &mut String, inputs: &[ShortInputAbi]) {
    if inputs.is_empty() {
        return;
    }

    for input in inputs {
        let rust_type = map_abi_type_to_rust_type(input.type_name.clone());
        buffer.push_str(&format!(
            "        let {} = {};\n",
            input.name,
            rust_type.get_default_value_expr()
        ));
    }

    buffer.push('\n');
}

fn write_contract_call_to_string(
    buffer: &mut String,
    endpoint_abi: &ShortEndpointAbi,
    name: &String,
) {
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
        endpoint_args_when_called_short(endpoint_abi.inputs.as_slice()),
        payment_snippet,
    ));
}

fn write_contract_query_to_string(
    buffer: &mut String,
    endpoint_abi: &ShortEndpointAbi,
    name: &String,
) {
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
        endpoint_args_when_called_short(endpoint_abi.inputs.as_slice()),
    ));
}

pub(crate) fn endpoint_args_when_called_short(inputs: &[ShortInputAbi]) -> String {
    let mut result = String::new();
    for input in inputs {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&input.name);
    }
    result
}
