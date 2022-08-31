use std::{
    fs::{self, File},
    io::Write,
};

use elrond_wasm::abi::ContractAbi;

use super::meta_config::MetaConfig;

const INIT_FUNC_NAME: &str = "init";

impl MetaConfig {
    // TODO: Handle overwrite flag
    pub fn generate_rust_snippets(&self) {
        if let Some(contract) = &self.main_contract {
            let crate_name = contract.output_base_name.clone().replace("-", "_");
            let file =
                create_snippets_crate_and_get_lib_file(&self.snippets_dir, &crate_name, true);
            write_snippets_to_file(file, &contract.abi, &crate_name);
        }
    }
}

#[must_use]
fn create_snippets_crate_and_get_lib_file(
    snippets_folder_path: &str,
    contract_crate_name: &str,
    overwrite: bool,
) -> File {
    create_snippets_folder(snippets_folder_path);
    create_snippets_gitignore(snippets_folder_path, overwrite);
    create_snippets_cargo_toml(snippets_folder_path, contract_crate_name, overwrite);
    create_src_folder(snippets_folder_path);
    create_and_get_lib_file(snippets_folder_path, overwrite)
}

fn create_snippets_folder(snippets_folder_path: &str) {
    // returns error if folder already exists, so we ignore the result
    let _ = fs::create_dir(snippets_folder_path);
}

fn create_snippets_gitignore(snippets_folder_path: &str, overwrite: bool) {
    let gitignore_path = format!("{}/.gitignore", snippets_folder_path);
    let mut file = if overwrite {
        File::create(&gitignore_path).unwrap()
    } else {
        match File::options().create_new(true).open(&gitignore_path) {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    writeln!(
        &mut file,
        "# Pem files are used for interactions, but shouldn't be committed
*.pem"
    )
    .unwrap();
}

fn create_snippets_cargo_toml(
    snippets_folder_path: &str,
    contract_crate_name: &str,
    overwrite: bool,
) {
    let cargo_toml_path = format!("{}/Cargo.toml", snippets_folder_path);
    let mut file = if overwrite {
        File::create(&cargo_toml_path).unwrap()
    } else {
        match File::options().create_new(true).open(&cargo_toml_path) {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    writeln!(
        &mut file,
        "[package]
name = \"rust-interact\"
version = \"0.0.0\"
authors = [\"you\"]
edition = \"2018\"
publish = false

[[bin]]
name = \"rust-interact\"
path = \"src/lib.rs\"

[dependencies.{}]
path = \"..\"

[dependencies.elrond-interact-snippets]
version = \"0.1.0\"
",
        contract_crate_name
    )
    .unwrap();
}

fn create_src_folder(snippets_folder_path: &str) {
    // returns error if folder already exists, so we ignore the result
    let src_folder_path = format!("{}/src", snippets_folder_path);
    let _ = fs::create_dir(src_folder_path);
}

#[must_use]
fn create_and_get_lib_file(snippets_folder_path: &str, overwrite: bool) -> File {
    let lib_path = format!("{}/src/lib.rs", snippets_folder_path);
    if overwrite {
        File::create(&lib_path).unwrap()
    } else {
        match File::options().create_new(true).open(&lib_path) {
            Ok(f) => f,
            Err(_) => panic!("lib.rs file already exists, overwrite option was not provided"),
        }
    }
}

fn write_snippets_to_file(mut file: File, abi: &ContractAbi, contract_crate_name: &str) {
    write_snippet_imports(&mut file, contract_crate_name);
    write_snippet_constants(&mut file);
    write_contract_type_alias(&mut file, contract_crate_name);
    write_snippet_main_function(&mut file, abi);
}

fn write_snippet_imports(file: &mut File, contract_crate_name: &str) {
    writeln!(
        file,
        "#[allow(non_snake_case)]

use {}::ProxyTrait as _;
use elrond_interact_snippets::{{
    elrond_wasm::{{
        elrond_codec::multi_types::MultiValueVec,
        storage::mappers::SingleValue,
        types::{{Address, CodeMetadata}},
    }},
    elrond_wasm_debug::{{
        bech32, mandos::interpret_trait::InterpreterContext, mandos_system::model::*, ContractInfo,
        DebugApi,
    }},
    env_logger,
    erdrs::interactors::wallet::Wallet,
    tokio, Interactor,
}};
use std::{{
    env::Args,
    io::{{Read, Write}},
}};",
        contract_crate_name
    )
    .unwrap();

    write_newline(file);
}

fn write_snippet_constants(file: &mut File) {
    writeln!(file, "const GATEWAY: &str = elrond_interact_snippets::erdrs::blockchain::rpc::DEVNET_GATEWAY;
const PEM: &str = \"alice.pem\";

const SYSTEM_SC_BECH32: &str = \"erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u\";
const DEFAULT_ADDRESS_EXPR: &str = \"0x0000000000000000000000000000000000000000000000000000000000000000\";").unwrap();

    write_newline(file);
}

fn write_contract_type_alias(file: &mut File, contract_crate_name: &str) {
    writeln!(
        file,
        "type ContractType = ContractInfo<{}::Proxy<DebugApi>>;",
        contract_crate_name
    )
    .unwrap();

    write_newline(file);
}

fn write_snippet_main_function(file: &mut File, abi: &ContractAbi) {
    writeln!(
        file,
        "#[tokio::main]
async fn main() {{
    env_logger::init();
    let _ = DebugApi::dummy();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect(\"at least one argument required\");
    let mut state = State::new().await;
    match cmd.as_str() {{"
    )
    .unwrap();

    // all contracts have a deploy and upgrade snippet
    writeln!(
        file,
        "        \"deploy\" => state.deploy().await,
        \"upgrade\" => state.upgrade().await,"
    )
    .unwrap();

    for endpoint in &abi.endpoints {
        if endpoint.name == INIT_FUNC_NAME {
            continue;
        }

        writeln!(
            file,
            "        \"{}\" => state.{}().await,",
            endpoint.name, endpoint.name
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

fn write_newline(file: &mut File) {
    file.write(b"\n").unwrap();
}
