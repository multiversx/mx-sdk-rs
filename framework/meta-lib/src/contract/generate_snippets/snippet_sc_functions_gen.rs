use std::{fs::File, io::Write};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, EndpointMutabilityAbi, InputAbi};

use super::{snippet_gen_common::write_newline, snippet_type_map::map_abi_type_to_rust_type};

pub(crate) fn write_interact_struct_impl(
    file: &mut File,
    abi: &ContractAbi,
    wasm_output_file_path_expr: &str,
) {
    writeln!(
        file,
        r#"impl ContractInteract {{
    async fn new() -> Self {{
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(test_wallets::alice());
        
        let contract_code = BytesValue::interpret_from(
            {},
            &InterpreterContext::default(),
        );

        ContractInteract {{
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state()
        }}
    }}
"#,
        wasm_output_file_path_expr,
    )
    .unwrap();

    write_deploy_method_impl(file, &abi.constructors[0], &abi.name);

    for endpoint_abi in &abi.endpoints {
        write_endpoint_impl(file, endpoint_abi, &abi.name);
    }

    // close impl block brackets
    writeln!(file, "}}").unwrap();
}

fn write_deploy_method_impl(file: &mut File, init_abi: &EndpointAbi, name: &String) {
    write_method_declaration(file, "deploy");
    write_endpoint_args_declaration(file, &init_abi.inputs);
    let proxy_name = format!("{}Proxy", name);

    writeln!(
        file,
        r#"        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .typed(proxy::{})
            .init({})
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

        println!("new address: {{new_address_bech32}}");"#,
        proxy_name,
        endpoint_args_when_called(init_abi.inputs.as_slice()),
    )
    .unwrap();

    // close method block brackets
    writeln!(file, "    }}").unwrap();
    write_newline(file);
}

fn write_endpoint_impl(file: &mut File, endpoint_abi: &EndpointAbi, name: &String) {
    write_method_declaration(file, &endpoint_abi.rust_method_name);
    write_payments_declaration(file, &endpoint_abi.payable_in_tokens);
    write_endpoint_args_declaration(file, &endpoint_abi.inputs);
    if matches!(endpoint_abi.mutability, EndpointMutabilityAbi::Readonly) {
        write_contract_query(file, endpoint_abi, name);
    } else {
        write_contract_call(file, endpoint_abi, name);
    }

    // close method block brackets
    writeln!(file, "    }}").unwrap();
    write_newline(file);
}

fn write_method_declaration(file: &mut File, endpoint_name: &str) {
    writeln!(file, "    async fn {endpoint_name}(&mut self) {{").unwrap();
}

fn write_payments_declaration(file: &mut File, accepted_tokens: &[String]) {
    if accepted_tokens.is_empty() {
        return;
    }

    // only handle EGLD and "any" case, as they're the most common
    let biguint_default = map_abi_type_to_rust_type("BigUint".to_string());
    let first_accepted = &accepted_tokens[0];
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
            "        let token_id = String::new();
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
        let rust_type = map_abi_type_to_rust_type(input.type_names.abi.clone());
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
        result.push_str(&input.arg_name);
    }
    result
}

fn write_contract_call(file: &mut File, endpoint_abi: &EndpointAbi, name: &String) {
    let payment_snippet = if endpoint_abi.payable_in_tokens.is_empty() {
        ""
    } else if endpoint_abi.payable_in_tokens[0] == "EGLD" {
        "\n            .egld(egld_amount)"
    } else {
        "\n            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))"
    };

    writeln!(
        file,
        r#"        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::{}Proxy)
            .{}({}){}
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {{response:?}}");"#,
        name,
        endpoint_abi.rust_method_name,
        endpoint_args_when_called(endpoint_abi.inputs.as_slice()),
        payment_snippet,
    )
    .unwrap();
}

fn write_contract_query(file: &mut File, endpoint_abi: &EndpointAbi, name: &String) {
    writeln!(
        file,
        r#"        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::{}Proxy)
            .{}({})
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {{result_value:?}}");"#,
        name,
        endpoint_abi.rust_method_name,
        endpoint_args_when_called(endpoint_abi.inputs.as_slice()),
    )
    .unwrap();
}
