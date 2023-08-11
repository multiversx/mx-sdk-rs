use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, EndpointMutabilityAbi, InputAbi, OutputAbi};

use super::{snippet_gen_common::write_newline, snippet_type_map::map_abi_type_to_rust_type};

pub(crate) fn write_state_struct_impl(
    file: &mut File,
    abi: &ContractAbi,
    wasm_output_file_path_expr: &str,
) {
    writeln!(
        file,
        r#"impl State {{
    async fn new() -> Self {{
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let sc_addr_expr = if SC_ADDRESS == "" {{
            DEFAULT_ADDRESS_EXPR.to_string()
        }} else {{
            "bech32:".to_string() + SC_ADDRESS
        }};
        let contract_code = BytesValue::interpret_from(
            {},
            &InterpreterContext::default(),
        );
        let contract = ContractType::new(sc_addr_expr);

        State {{
            interactor,
            wallet_address,
            contract_code,
            contract,
        }}
    }}
"#,
        wasm_output_file_path_expr,
    )
    .unwrap();

    write_deploy_method_impl(file, &abi.constructors[0]);

    for endpoint_abi in &abi.endpoints {
        write_endpoint_impl(file, endpoint_abi);
    }

    // close impl block brackets
    writeln!(file, "}}").unwrap();
}

fn write_deploy_method_impl(file: &mut File, init_abi: &EndpointAbi) {
    write_method_declaration(file, "deploy");
    write_endpoint_args_declaration(file, &init_abi.inputs);

    let output_type = map_output_types_to_rust_types(&init_abi.outputs);
    writeln!(
        file,
        r#"        let (new_address, _) = self
            .interactor
            .sc_deploy_get_result::<_, {}>(
                ScDeployStep::new()
                    .call(self.contract.{}({}))
                    .from(&self.wallet_address)
                    .code(&self.contract_code)
                    .expect(TxExpect::ok().additional_error_message("deploy failed: ")),
            )
            .await;
s
        let new_address_bech32 = bech32::encode(&new_address);
        println!("new address: {{new_address_bech32}}");"#,
        output_type,
        init_abi.rust_method_name,
        endpoint_args_when_called(init_abi.inputs.as_slice()),
    )
    .unwrap();

    // close method block brackets
    writeln!(file, "    }}").unwrap();
    write_newline(file);
}

fn write_endpoint_impl(file: &mut File, endpoint_abi: &EndpointAbi) {
    write_method_declaration(file, endpoint_abi.rust_method_name);
    write_payments_declaration(file, endpoint_abi.payable_in_tokens);
    write_endpoint_args_declaration(file, &endpoint_abi.inputs);
    if matches!(endpoint_abi.mutability, EndpointMutabilityAbi::Readonly) {
        write_contract_query(file, endpoint_abi);
    } else {
        write_contract_call(file, endpoint_abi);
    }

    // close method block brackets
    writeln!(file, "    }}").unwrap();
    write_newline(file);
}

fn write_method_declaration(file: &mut File, endpoint_name: &str) {
    writeln!(file, "    async fn {endpoint_name}(&mut self) {{").unwrap();
}

fn write_payments_declaration(file: &mut File, accepted_tokens: &[&str]) {
    if accepted_tokens.is_empty() {
        return;
    }

    // only handle EGLD and "any" case, as they're the most common
    let biguint_default = map_abi_type_to_rust_type("BigUint".to_string());
    let first_accepted = accepted_tokens[0];
    if first_accepted == "EGLD" {
        writeln!(
            file,
            "        let egld_amount = {};",
            biguint_default.get_default_value_expr()
        )
        .unwrap();
    } else {
        writeln!(
            file,
            "        let token_id = b\"\";
        let token_nonce = 0u64;
        let token_amount = {};",
            biguint_default.get_default_value_expr()
        )
        .unwrap();
    }

    write_newline(file);
}

fn write_endpoint_args_declaration(file: &mut File, inputs: &[InputAbi]) {
    if inputs.is_empty() {
        return;
    }

    for input in inputs {
        let rust_type = map_abi_type_to_rust_type(input.type_name.clone());
        writeln!(
            file,
            "        let {} = {};",
            input.arg_name,
            rust_type.get_default_value_expr()
        )
        .unwrap();
    }

    write_newline(file);
}

fn endpoint_args_when_called(inputs: &[InputAbi]) -> String {
    let mut result = String::new();
    for input in inputs {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(input.arg_name);
    }
    result
}

fn write_contract_call(file: &mut File, endpoint_abi: &EndpointAbi) {
    let payment_snippet = if endpoint_abi.payable_in_tokens.is_empty() {
        ""
    } else if endpoint_abi.payable_in_tokens[0] == "EGLD" {
        "\n                    .egld_value(egld_amount)"
    } else {
        "\n                    .esdt_transfer(token_id.to_vec(), token_nonce, token_amount)"
    };

    let output_type = map_output_types_to_rust_types(&endpoint_abi.outputs);
    writeln!(
        file,
        r#"        let response: TypedResponse<{}> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.{}({}))
                    .from(&self.wallet_address){}
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {{result:?}}");"#,
        output_type,
        endpoint_abi.rust_method_name,
        endpoint_args_when_called(endpoint_abi.inputs.as_slice()),
        payment_snippet,
    )
    .unwrap();
}

fn write_contract_query(file: &mut File, endpoint_abi: &EndpointAbi) {
    let output_type = map_output_types_to_rust_types(&endpoint_abi.outputs);
    writeln!(
        file,
        r#"        let result_value: {} = self
            .interactor
            .vm_query(self.contract.{}({}))
            .await;
"#,
        output_type,
        endpoint_abi.rust_method_name,
        endpoint_args_when_called(endpoint_abi.inputs.as_slice()),
    )
    .unwrap();
}

fn map_output_types_to_rust_types(outputs: &[OutputAbi]) -> String {
    let results_len = outputs.len();
    if results_len == 0 {
        return "()".to_string();
    }

    // format to be the same as when multi-value is an argument
    // for results, each type is a different array entry
    let mut input_str = String::new();
    if results_len > 1 {
        input_str += "multi";
        input_str += "<";
    }

    for (i, output) in outputs.iter().enumerate() {
        input_str += &output.type_name;

        if i < results_len - 1 {
            input_str += ",";
        }
    }

    if results_len > 1 {
        input_str += ">";
    }

    let output_rust_type = map_abi_type_to_rust_type(input_str);
    output_rust_type.get_type_name().to_string()
}
