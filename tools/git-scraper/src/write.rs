use crate::fetch::{
    fetch_directory_contents, fetch_file_content, fetch_interactor_contents, fetch_meta_contents,
};
use reqwest::blocking::Client;
use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};

pub(crate) fn write_instructions(writer: &mut BufWriter<File>) -> io::Result<()> {
    let instructions = r#"INSTRUCTIONS FOR USING THIS FILE
==============================
1. Each contract is separated by '////////////////////////'. The end of this section is also marked by '////////////////////////'.
2. For each contract you will find:
   - NAME: The contract's folder name
   - DESCRIPTION: Content from README.md
   - SRC FOLDER: All source files
   - CARGO.TOML: Dependencies and contract configuration
   - INTERACTOR FOLDER: If available, contains interactor files (used for deployment and interaction on the blockchain)
3. Before the contract code dump you will find a step by step description of how to create, build and deploy smart contracts on MultiversX
4. All contracts contain a multiversx.json file with the following content:
{
    "language": "rust"
}
This file is mandatory for sc-meta to recognize our contract crate. You should generate it for all contracts.
5. If the contract contains an interactor/ folder, the file structure should change in order to compile. The contract is by default #[no_std],
while the interactor uses std, so they cannot be compiled at once. We need an external workspace containing both of them in order to compile.
For example, if we have adder/ contract which contains adder/interactor, we should create an outer workspace containing the adder, as such:

Cargo.toml

[workspace]
resolver = "2"
members = ["adder", "adder/meta", "adder/interactor]

Folder structure:

outer-folder/
|---> adder/
        |---> src/
        |---> Cargo.toml
        |---> interactor/
                |---> src/
                |---> Cargo.toml
|---> Cargo.toml

INSTRUCTIONS FOR CREATING, BUILDING AND DEPLOYING SMART CONTRACTS ON MULTIVERSX
==============================
1. Considering environment, the only critical components a developer should install are:
- rust (using rustup for better version management, as recommended on rust-lang.org): 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
After installing rust, use the `stable` toolchain:
```bash
rustup update
rustup default stable
```
- sc-meta tool:
```bash
cargo install multiversx-sc-meta --locked
```
Once sc-meta is ready, install the wasm32 target (for the Rust compiler), wasm-opt, and others dependencies as follows:
```bash
# Installs `wasm32`, `wasm-opt`, and others in one go:
sc-meta install all
cargo install twiggy
```
If the dependencies installation fails (sc-meta install all) use `sc-meta install [dependency-name]` to install dependencies one by one.

2. In order to start writing a smart contract from an empty folder, the easiest way is to use the sc-meta template feature with the `new` command:
```bash
sc-meta new --template empty --name my-contract
```
This will create a new project with the following structure:
- src/
  - lib.rs (main contract file, must include #[multiversx_sc::contract] attribute)
- wasm/
- Cargo.toml

3. After creating a template, a developer should start writing rust code in src/. src/lib.rs is the main file, you can also create other rust files
as modules marked with #[multiversx_sc::module] and import them in the main file).

Key requirements:
- Contract must have #[init] function for deployment
- Public functions need #[endpoint] attribute
- Storage mappers (like SingleValueMapper, UnorderedSetMapper) need #[storage_mapper] and #[view] for easier access API
- Use MultiversX SC types (BigUint, ManagedBuffer, etc.)
- Split code into modules using #[multiversx_sc::module] for better organization

4. After the code is written, it should first compile. A quick `cargo check` can verify the compilation.

5. If the code compiles, it is time to build the contract. A contract should build without errors (and preferably warnings):
```bash
sc-meta all build
```
This will generate:
- wasm/my-contract.wasm (the contract bytecode)
- wasm/my-contract.mxsc.json (contract metadata)

6. After the build is done, we can use the interactor to deploy the contract. Generate it with:
```bash
sc-meta all snippets
```
The interactor allows you to:
- Configure your wallet (use wallets from test_wallet for easier devnet deployment)
- Choose network (devnet/testnet/mainnet)
- Set gas limits
- Send deploy/upgrade transactions for your contract through Rust functions
- Call contract endpoints with arguments through Rust functions

In short:
- env installation (rust and sc-meta)
- sc-meta new --template empty --name my-contract (new contract from template)
- write rust code inside src/ (remember required attributes)
- cargo check
- sc-meta all build
- write interactor code/ generate using sc-meta all snippets
- deploy the contract on devnet using the interactor (recommended for testing, no real EGLD needed)

Common issues:
- Missing contract/module attributes
- Incorrect types in function arguments
- Storage not properly initialized
- Gas limits too low
- Missing endpoint attributes

////////////////////////
"#;

    writeln!(writer, "{}", instructions)?;
    writer.flush()?;
    Ok(())
}

pub(crate) fn write_readme(
    client: &Client,
    folder_url: &str,
    writer: &mut BufWriter<File>,
    folder_name: &str,
) -> io::Result<()> {
    if let Some(readme_content) = fetch_file_content(client, folder_url, "README.md") {
        writeln!(writer, "\nDESCRIPTION:\n{}", readme_content)?;
    } else {
        writeln!(writer, "\nDESCRIPTION:\nNone")?;
        println!("No README.md available for {}", folder_name);
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn write_src_folder(
    client: &Client,
    folder_url: &str,
    writer: &mut BufWriter<File>,
    folder_name: &str,
) -> io::Result<()> {
    writeln!(writer, "\nSRC FOLDER:")?;
    if let Some(src_files) = fetch_directory_contents(client, folder_url, "src") {
        for (file_name, file_content) in src_files {
            writeln!(writer, "FILE_NAME: {}", file_name)?;
            writeln!(writer, "{}", file_content)?;
        }
    } else {
        writeln!(writer, "No src folder found")?;
        println!("No src folder found for {}", folder_name);
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn write_cargo_toml(
    client: &Client,
    folder_url: &str,
    writer: &mut BufWriter<File>,
    folder_name: &str,
) -> io::Result<()> {
    if let Some(cargo_content) = fetch_file_content(client, folder_url, "Cargo.toml") {
        writeln!(writer, "\nCARGO.TOML:")?;
        writeln!(writer, "{}", cargo_content)?;
    } else {
        println!("No Cargo.toml found for {}", folder_name);
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn write_interactor_files(
    client: &Client,
    folder_url: &str,
    writer: &mut BufWriter<File>,
    folder_name: &str,
) -> io::Result<()> {
    if let Some((src_files, cargo_content)) = fetch_interactor_contents(client, folder_url) {
        writeln!(writer, "\nINTERACTOR FOLDER:")?;

        if !src_files.is_empty() {
            writeln!(writer, "SRC FOLDER:")?;
            for (file_name, file_content) in src_files {
                writeln!(writer, "FILE_NAME: {}", file_name)?;
                writeln!(writer, "{}", file_content)?;
            }
        }

        if let Some(cargo_content) = cargo_content {
            writeln!(writer, "\nINTERACTOR CARGO.TOML:")?;
            writeln!(writer, "{}", cargo_content)?;
        }
    } else {
        println!("No interactor folder found for {}", folder_name);
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn write_meta_folder(
    client: &Client,
    folder_url: &str,
    writer: &mut BufWriter<File>,
    folder_name: &str,
) -> io::Result<()> {
    if let Some((src_files, cargo_content)) = fetch_meta_contents(client, folder_url) {
        writeln!(writer, "\nMETA FOLDER:")?;

        if !src_files.is_empty() {
            writeln!(writer, "SRC FOLDER:")?;
            for (file_name, file_content) in src_files {
                writeln!(writer, "FILE_NAME: {}", file_name)?;
                writeln!(writer, "{}", file_content)?;
            }
        }

        if let Some(cargo_content) = cargo_content {
            writeln!(writer, "\nMETA CARGO.TOML:")?;
            writeln!(writer, "{}", cargo_content)?;
        }
    } else {
        println!("No meta folder found for {}", folder_name);
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn cleanup_local_paths(file_path: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut skip_next = false;

    for line in &lines {
        if skip_next {
            skip_next = false;
            continue;
        }

        if line.trim().starts_with("path = \"../../../") {
            continue;
        }

        if line.trim().starts_with("version = ") {
            skip_next = has_local_path_next(&lines, line);
        }

        result.push(*line);
    }

    fs::write(file_path, result.join("\n"))?;
    Ok(())
}

fn has_local_path_next(lines: &[&str], current: &str) -> bool {
    lines
        .iter()
        .skip_while(|&&x| x != current)
        .nth(1)
        .map_or(false, |next| next.trim().starts_with("path = \"../../../"))
}
