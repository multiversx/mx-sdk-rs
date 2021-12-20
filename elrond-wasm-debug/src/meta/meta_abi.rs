use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use crate::abi_json::{serialize_abi_to_json, ContractAbiJson};

use super::meta_config::{ContractMetadata, MetaConfig};

fn write_contract_abi(contract_metadata: &ContractMetadata, output_path: &str) {
    let abi_json = ContractAbiJson::from(&contract_metadata.abi);
    let abi_string = serialize_abi_to_json(&abi_json);

    let abi_file_path = format!("{}/{}", output_path, contract_metadata.abi_output_name(),);
    let mut abi_file = File::create(abi_file_path).unwrap();
    write!(abi_file, "{}", abi_string).unwrap();
}

impl MetaConfig {
    pub fn write_abi(&self) {
        create_dir_all(&self.output_dir).unwrap();

        if let Some(main_contract) = &self.main_contract {
            write_contract_abi(main_contract, self.output_dir.as_str());
            main_contract.create_dir_all();
        }

        if let Some(view_contract) = &self.view_contract {
            write_contract_abi(view_contract, self.output_dir.as_str());
            view_contract.create_dir_all();
        }
    }
}
