use std::{fs::File, io::Write};

use multiversx_sc::abi::ContractAbi;

use super::snippet_gen_common::write_newline;

pub(crate) fn write_snippet_imports(file: &mut File) {
    writeln!(
        file,
        "#![allow(non_snake_case)]

mod config;
mod proxy;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{{Deserialize, Serialize}};
use std::{{
    io::{{Read, Write}},
    path::Path,
}};"
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_snippet_constants(file: &mut File) {
    writeln!(file, "const STATE_FILE: &str = \"state.toml\";").unwrap();
}

pub(crate) fn write_snippet_main_function(file: &mut File, abi: &ContractAbi, crate_name: &str) {
    writeln!(
        file,
        "
pub async fn {crate_name}_cli() {{
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect(\"at least one argument required\");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {{"
    )
    .unwrap();

    // all contracts have a deploy snippet
    writeln!(file, r#"        "deploy" => interact.deploy().await,"#).unwrap();

    for upgrade_endpoint in &abi.upgrade_constructors {
        writeln!(
            file,
            r#"        "{}" => interact.{}().await,"#,
            upgrade_endpoint.name, upgrade_endpoint.rust_method_name
        )
        .unwrap();
    }

    for endpoint in &abi.endpoints {
        writeln!(
            file,
            r#"        "{}" => interact.{}().await,"#,
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
}

pub(crate) fn write_interact_struct_declaration(file: &mut File) {
    writeln!(
        file,
        "pub struct ContractInteract {{
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State
}}"
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_state_struct_declaration(file: &mut File) {
    writeln!(
        file,
        "
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {{
    contract_address: Option<Bech32Address>
}}"
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_snippet_state_impl(file: &mut File) {
    writeln!(
        file,
        r#"impl State {{
        // Deserializes state from file
        pub fn load_state() -> Self {{
            if Path::new(STATE_FILE).exists() {{
                let mut file = std::fs::File::open(STATE_FILE).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                toml::from_str(&content).unwrap()
            }} else {{
                Self::default()
            }}
        }}
    
        /// Sets the contract address
        pub fn set_address(&mut self, address: Bech32Address) {{
            self.contract_address = Some(address);
        }}
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {{
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
        }}
    }}
    
    impl Drop for State {{
        // Serializes state to file
        fn drop(&mut self) {{
            let mut file = std::fs::File::create(STATE_FILE).unwrap();
            file.write_all(toml::to_string(self).unwrap().as_bytes())
                .unwrap();
        }}
    }}"#
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_config_imports(file: &mut File) {
    writeln!(
        file,
        "#![allow(unused)]

use serde::Deserialize;
use std::io::Read;
"
    )
    .unwrap();
}

pub(crate) fn write_config_constants(file: &mut File) {
    writeln!(
        file,
        "/// Config file
const CONFIG_FILE: &str = \"config.toml\";
"
    )
    .unwrap();
}

pub(crate) fn write_config_struct_declaration(file: &mut File) {
    writeln!(
        file,
        r#"#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChainType {{
    Real,
    Simulator,
}}

/// Contract Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {{
    pub gateway_uri: String,
    pub chain_type: ChainType,
}}
"#
    )
    .unwrap();
}

pub(crate) fn write_config_struct_impl(file: &mut File) {
    writeln!(
        file,
        r#"impl Config {{
    // Deserializes config from file
    pub fn new() -> Self {{
        let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    }}

    pub fn chain_simulator_config() -> Self {{
        Config {{
            gateway_uri: "http://localhost:8085".to_owned(),
            chain_type: ChainType::Simulator,
        }}
    }}

    // Returns the gateway URI
    pub fn gateway_uri(&self) -> &str {{
        &self.gateway_uri
    }}

    // Returns if chain type is chain simulator
    pub fn use_chain_simulator(&self) -> bool {{
        match self.chain_type {{
            ChainType::Real => false,
            ChainType::Simulator => true,
        }}
    }}
}}"#
    )
    .unwrap();
}

pub(crate) fn write_chain_sim_test_to_file(file: &mut File, crate_name: &str) {
    writeln!(
        file,
        r#"use multiversx_sc_snippets::imports::*;
use rust_interact::ContractInteract;

// Simple deploy test that runs using the chain simulator configuration.
// In order for this test to work, make sure that the `config.toml` file contains the chain simulator config (or choose it manually)
// The chain simulator should already be installed and running before attempting to run this test.
// The chain-simulator-tests feature should be present in Cargo.toml.
// Can be run with `cargo test --features chain-simulator-tests`.
#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn deploy_test_{crate_name}_cs() {{
    let mut interactor = ContractInteract::new().await;

    interactor.deploy().await;
}}"#
    ).unwrap()
}

pub(crate) fn write_interactor_test_to_file(file: &mut File, crate_name: &str) {
    writeln!(
        file,
        r#"use multiversx_sc_snippets::imports::*;
use rust_interact::ContractInteract;

// Simple deploy test that runs on the real blockchain configuration.
// In order for this test to work, make sure that the `config.toml` file contains the real blockchain config (or choose it manually)
// Can be run with `cargo test`.
#[tokio::test]
async fn deploy_test_{crate_name}() {{
    let mut interactor = ContractInteract::new().await;

    interactor.deploy().await;
}}"#
    ).unwrap()
}
