use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use elrond_wasm::contract_base::ContractAbiProvider;

const WASM_SRC_DIR: &str = "../wasm/src";
const WASM_SRC_PATH: &str = "../wasm/src/lib.rs";

const PRELUDE: &str = "////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;
";

fn write_endpoint(wasm_lib_file: &mut File, endpoint_name: &str) {
    writeln!(
        wasm_lib_file,
        "
#[no_mangle]
pub fn {}() {{
    use_module::endpoints::{}(elrond_wasm_node::arwen_api());
}}",
        endpoint_name, endpoint_name
    )
    .unwrap();
}

pub fn write_wasm_lib<AbiObj: ContractAbiProvider>() {
    let abi = <AbiObj as ContractAbiProvider>::abi();
    create_dir_all(WASM_SRC_DIR).unwrap();
    let mut wasm_lib_file = File::create(WASM_SRC_PATH).unwrap();
    wasm_lib_file.write_all(PRELUDE.as_bytes()).unwrap();

    for endpoint in &abi.endpoints {
        write_endpoint(&mut wasm_lib_file, endpoint.name);
    }

    write_endpoint(&mut wasm_lib_file, "callBack");
}
