use std::{fs::File, io::Write};

use multiversx_sc::abi::ContractAbi;

use super::snippet_gen_common::write_newline;

pub(crate) fn write_snippet_imports(file: &mut File, contract_crate_name: &str) {
    writeln!(
        file,
        "#![allow(non_snake_case)]

use {contract_crate_name}::ProxyTrait as _;
use {contract_crate_name}::*;

use multiversx_sc_snippets::{{
    env_logger,
    erdrs::wallet::Wallet,
    multiversx_sc::{{codec::multi_types::*, types::*}},
    multiversx_sc_scenario::{{
        api::StaticApi,
        bech32,
        scenario_format::interpret_trait::{{InterpretableFrom, InterpreterContext}},
        scenario_model::*,
        ContractInfo,
    }},
    sdk, tokio, Interactor,
}};
"
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_snippet_constants(file: &mut File) {
    writeln!(
        file,
        "const GATEWAY: &str = sdk::blockchain::DEVNET_GATEWAY;
const PEM: &str = \"alice.pem\";
const SC_ADDRESS: &str = \"\";

const SYSTEM_SC_BECH32: &str = \"erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u\";
const DEFAULT_ADDRESS_EXPR: &str =
    \"0x0000000000000000000000000000000000000000000000000000000000000000\";
const TOKEN_ISSUE_COST: u64 = 50_000_000_000_000_000;"
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_contract_type_alias(file: &mut File, contract_crate_name: &str) {
    writeln!(
        file,
        "type ContractType = ContractInfo<{contract_crate_name}::Proxy<StaticApi>>;"
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_snippet_main_function(file: &mut File, abi: &ContractAbi) {
    writeln!(
        file,
        "#[tokio::main]
async fn main() {{
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect(\"at least one argument required\");
    let mut state = State::new().await;
    match cmd.as_str() {{"
    )
    .unwrap();

    // all contracts have a deploy snippet
    writeln!(file, r#"        "deploy" => state.deploy().await,"#).unwrap();

    for endpoint in &abi.endpoints {
        writeln!(
            file,
            r#"        "{}" => state.{}().await,"#,
            endpoint.name, endpoint.rust_method_name
        )
        .unwrap();
    }

    // general case of "command not found" + close curly brackets
    writeln!(
        file,
        "        _ => panic!(\"unknown command: {{}}\", &cmd),
    }}
}}"
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_state_struct_declaration(file: &mut File) {
    writeln!(
        file,
        "struct State {{
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    contract: ContractType,
}}"
    )
    .unwrap();

    write_newline(file);
}
