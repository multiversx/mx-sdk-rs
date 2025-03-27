use std::{fs::File, io::Write, path::Path};

use super::snippet_abi_check::{write_endpoint_impl_to_string, ShortContractAbi};

pub(crate) const DEFAULT_GAS: &str = "30_000_000u64";

pub(crate) fn write_interact_struct_impl(
    file: &mut File,
    abi: &ShortContractAbi,
    crate_name: &str,
) {
    let crate_path = crate_name.replace("_", "-");
    let mxsc_file_name = format!("{crate_path}.mxsc.json");
    let wasm_output_file_path = Path::new("..").join("output").join(mxsc_file_name);

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
        interactor.generate_blocks_until_epoch(1).await.unwrap();
        
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

    let mut buffer = String::new();
    write_endpoint_impl_to_string(&mut buffer, &abi.constructor[0], &abi.name);

    for upgrade_abi in &abi.upgrade_constructor {
        write_endpoint_impl_to_string(&mut buffer, upgrade_abi, &abi.name);
    }

    for endpoint_abi in &abi.endpoints {
        write_endpoint_impl_to_string(&mut buffer, endpoint_abi, &abi.name);
    }

    write!(file, "{}", buffer).unwrap();

    // close impl block brackets
    writeln!(file, "}}").unwrap();
}
