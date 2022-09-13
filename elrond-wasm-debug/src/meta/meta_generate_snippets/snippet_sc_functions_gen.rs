use std::{fs::File, io::Write};

use elrond_wasm::abi::{ContractAbi, EndpointAbi, EndpointMutabilityAbi, InputAbi, OutputAbi};

use super::snippet_gen_common::write_newline;

pub(crate) fn write_state_struct_impl(
    file: &mut File,
    abi: &ContractAbi,
    wasm_output_file_path_expr: &str,
) {
    writeln!(
        file,
        "impl State {{
    async fn new() -> Self {{
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let sc_addr_expr = if SC_ADDRESS == \"\" {{
            DEFAULT_ADDRESS_EXPR.to_string()
        }} else {{
            \"bech32:\".to_string() + SC_ADDRESS
        }};
        let contract = ContractType::new(sc_addr_expr);

        State {{
            interactor,
            wallet_address,
            contract,
        }}
    }}\n"
    )
    .unwrap();

    write_deploy_method_impl(file, &abi.constructors[0], wasm_output_file_path_expr);

    for endpoint_abi in &abi.endpoints {
        write_endpoint_impl(file, endpoint_abi);
    }

    // close impl block brackets
    writeln!(file, "}}").unwrap();
}

fn write_deploy_method_impl(
    file: &mut File,
    init_abi: &EndpointAbi,
    wasm_output_file_path_expr: &str,
) {
    write_method_declaration(file, "deploy");
    write_endpoint_args_declaration(file, &init_abi.inputs);

    writeln!(
        file,
        r#"        let result: elrond_interact_snippets::InteractorResult<PlaceholderOutput> = self
            .interactor
            .sc_deploy(
                self.contract
                    .{}({})
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .code_metadata(CodeMetadata::all())
                    .contract_code({}, &InterpreterContext::default())
                    .gas_limit(DEFAULT_GAS_LIMIT),
            )
            .await;

        let new_address = result.new_deployed_address();
        let new_address_bech32 = bech32::encode(&new_address);
        println!("new address: {{}}", new_address_bech32);
        let result_value = result.value();
"#,
        init_abi.rust_method_name,
        endpoint_args_when_called(init_abi.inputs.as_slice()),
        wasm_output_file_path_expr
    )
    .unwrap();

    write_call_results_print(file, &init_abi.outputs);

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
    write_call_results_print(file, &endpoint_abi.outputs);

    // close method block brackets
    writeln!(file, "    }}").unwrap();
    write_newline(file);
}

fn write_method_declaration(file: &mut File, endpoint_name: &str) {
    writeln!(file, "    async fn {}(&mut self) {{", endpoint_name).unwrap();
}

fn write_payments_declaration(file: &mut File, accepted_tokens: &[&str]) {
    if accepted_tokens.is_empty() {
        return;
    }

    // only handle EGLD and "any" case, as they're the most common
    let first_accepted = accepted_tokens[0];
    if first_accepted == "EGLD" {
        writeln!(file, "        let egld_amount = 0u64;").unwrap();
    } else {
        writeln!(
            file,
            "        let token_id = b\"\";
        let token_nonce = 0u64;
        let token_amount = 0u64;"
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
        writeln!(file, "        let {} = PlaceholderInput;", input.arg_name).unwrap();
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
        "\n            .egld_value(egld_amount)\n"
    } else {
        "\n            .esdt_transfer(token_id, token_nonce, token_amount)\n"
    };

    writeln!(
        file,
        r#"        let result: elrond_interact_snippets::InteractorResult<PlaceholderOutput> = self
            .interactor
            .sc_call_get_result(
                self.contract
                    .{}({})
                    .into_blockchain_call()
                    .from(&self.wallet_address){}
                    .gas_limit(DEFAULT_GAS_LIMIT)
                    .into(),
            )
            .await;
        let result_value = result.value();
"#,
        endpoint_abi.rust_method_name,
        endpoint_args_when_called(endpoint_abi.inputs.as_slice()),
        payment_snippet,
    )
    .unwrap();
}

fn write_contract_query(file: &mut File, endpoint_abi: &EndpointAbi) {
    writeln!(
        file,
        r#"        let result_value: PlaceholderOutput = self
            .interactor
            .vm_query(self.contract.{}({}))
            .await;
"#,
        endpoint_abi.rust_method_name,
        endpoint_args_when_called(endpoint_abi.inputs.as_slice()),
    )
    .unwrap();
}

fn write_call_results_print(file: &mut File, _outputs: &[OutputAbi]) {
    writeln!(file, r#"        println!("Result: {{:?}}", result_value);"#).unwrap();
}
