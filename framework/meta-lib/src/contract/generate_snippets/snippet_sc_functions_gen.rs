use std::{fs::File, io::Write, path::Path};

use multiversx_sc::abi::{ContractAbi, EndpointAbi, EndpointMutabilityAbi, InputAbi};

use super::{snippet_gen_common::write_newline, snippet_type_map::map_abi_type_to_rust_type};

const DEFAULT_GAS: &str = "30_000_000u64";

pub(crate) fn write_interact_struct_impl(file: &mut File, abi: &ContractAbi, crate_name: &str) {
    let crate_path = crate_name.replace("_", "-");
    let mxsc_file_name = format!("{crate_path}.mxsc.json");
    let wasm_output_file_path = Path::new("..").join("output").join(mxsc_file_name);
    let proxy_file_name = format!("{}_proxy", crate_name.replace("-", "_"));
    let proxy = format!("{}::{}Proxy", proxy_file_name, abi.name);

    let wasm_output_file_path_expr =
        format!("\"mxsc:{}\"", &wasm_output_file_path.to_string_lossy());

    writeln!(
        file,
        r#"impl ContractInteract {{
    pub async fn new(config: Config) -> Self {{
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("{}");
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_all_activations().await;
        
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
        crate_path, wasm_output_file_path_expr,
    )
    .unwrap();

    write_deploy_method_impl(file, &proxy, &abi.constructors[0]);

    for upgrade_abi in &abi.upgrade_constructors {
        write_upgrade_endpoint_impl(file, &proxy, upgrade_abi);
    }

    for endpoint_abi in &abi.endpoints {
        write_endpoint_impl(file, &proxy, endpoint_abi);
    }

    // close impl block brackets
    writeln!(file, "}}").unwrap();
}

fn write_deploy_method_impl(file: &mut File, proxy: &str, init_abi: &EndpointAbi) {
    write_method_declaration(file, "deploy");
    write_endpoint_args_declaration(file, &init_abi.inputs);

    writeln!(
        file,
        r#"        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas({DEFAULT_GAS})
            .typed({proxy})
            .init({})
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = new_address.to_bech32_default();
        println!("new address: {{new_address_bech32}}");
        self.state.set_address(new_address_bech32);"#,
        endpoint_args_when_called(init_abi.inputs.as_slice()),
    )
    .unwrap();

    // close method block brackets
    writeln!(file, "    }}").unwrap();
    write_newline(file);
}

fn write_upgrade_endpoint_impl(file: &mut File, proxy: &str, upgrade_abi: &EndpointAbi) {
    write_method_declaration(file, "upgrade");
    write_endpoint_args_declaration(file, &upgrade_abi.inputs);

    writeln!(
        file,
        r#"        let response = self
            .interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.wallet_address)
            .gas({DEFAULT_GAS})
            .typed({proxy})
            .upgrade({})
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {{response:?}}");"#,
        endpoint_args_when_called(upgrade_abi.inputs.as_slice()),
    )
    .unwrap();

    // close method block brackets
    writeln!(file, "    }}").unwrap();
    write_newline(file);
}

fn write_endpoint_impl(file: &mut File, proxy: &str, endpoint_abi: &EndpointAbi) {
    write_method_declaration(file, &endpoint_abi.rust_method_name);
    write_payments_declaration(file, &endpoint_abi.payable_in_tokens);
    write_endpoint_args_declaration(file, &endpoint_abi.inputs);
    if matches!(endpoint_abi.mutability, EndpointMutabilityAbi::Readonly) {
        write_contract_query(file, proxy, endpoint_abi);
    } else {
        write_contract_call(file, proxy, endpoint_abi);
    }

    // close method block brackets
    writeln!(file, "    }}").unwrap();
    write_newline(file);
}

fn write_method_declaration(file: &mut File, endpoint_name: &str) {
    writeln!(file, "    pub async fn {endpoint_name}(&mut self) {{").unwrap();
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

fn write_contract_call(file: &mut File, proxy: &str, endpoint_abi: &EndpointAbi) {
    let payment_snippet = if endpoint_abi.payable_in_tokens.is_empty() {
        ""
    } else if endpoint_abi.payable_in_tokens[0] == "EGLD" {
        "\n            .egld(egld_amount)"
    } else {
        "\n            .payment((EsdtTokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))"
    };

    writeln!(
        file,
        r#"        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas({DEFAULT_GAS})
            .typed({proxy})
            .{}({}){}
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {{response:?}}");"#,
        endpoint_abi.rust_method_name,
        endpoint_args_when_called(endpoint_abi.inputs.as_slice()),
        payment_snippet,
    )
    .unwrap();
}

fn write_contract_query(file: &mut File, proxy: &str, endpoint_abi: &EndpointAbi) {
    writeln!(
        file,
        r#"        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed({proxy})
            .{}({})
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {{result_value:?}}");"#,
        endpoint_abi.rust_method_name,
        endpoint_args_when_called(endpoint_abi.inputs.as_slice()),
    )
    .unwrap();
}
