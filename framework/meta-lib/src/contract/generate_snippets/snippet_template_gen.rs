use std::{fs::File, io::Write};

use multiversx_sc::abi::ContractAbi;

use super::snippet_gen_common::write_newline;

pub(crate) fn write_snippet_imports(file: &mut File) {
    writeln!(
        file,
        "#![allow(non_snake_case)]

mod proxy;

use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk;
use serde::{{Deserialize, Serialize}};
use std::{{
    io::{{Read, Write}},
    path::Path,
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
const STATE_FILE: &str = \"state.toml\";
"
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

    write_newline(file);
}

pub(crate) fn write_interact_struct_declaration(file: &mut File) {
    writeln!(
        file,
        "struct ContractInteract {{
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
struct State {{
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
