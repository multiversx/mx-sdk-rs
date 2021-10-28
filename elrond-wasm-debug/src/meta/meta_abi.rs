use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use elrond_wasm::abi::ContractAbi;

use crate::abi_json::{serialize_abi_to_json, ContractAbiJson};

pub fn write_abi(abi: &ContractAbi) {
    let abi_json = ContractAbiJson::from(abi);
    let abi_string = serialize_abi_to_json(&abi_json);

    create_dir_all("../output").unwrap();
    let abi_file_path = format!("../output/{}.abi.json", &abi.build_info.contract_crate.name);
    let mut abi_file = File::create(abi_file_path).unwrap();
    write!(abi_file, "{}", abi_string).unwrap();
}
