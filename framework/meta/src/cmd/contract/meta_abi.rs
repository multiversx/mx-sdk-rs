use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use crate::abi_json::{serialize_abi_to_json, ContractAbiJson};

use super::{meta_config::MetaConfig, output_contract::OutputContract};

fn write_contract_abi(output_contract: &OutputContract, git_version: &str, output_path: &str) {
    let mut abi_json = ContractAbiJson::from(&output_contract.abi);
    if let Some(build_info) = &mut abi_json.build_info {
        build_info.contract_crate.git_version = git_version.to_string();
    }
    let abi_string = serialize_abi_to_json(&abi_json);

    let abi_file_path = format!("{output_path}/{}", output_contract.abi_output_name(),);
    let mut abi_file = File::create(abi_file_path).unwrap();
    write!(abi_file, "{abi_string}").unwrap();
}

impl MetaConfig {
    pub fn write_abi(&self) {
        create_dir_all(&self.output_dir).unwrap();
        let git_version = self.git_describe();
        for output_contract in &self.output_contracts.contracts {
            write_contract_abi(
                output_contract,
                git_version.as_str(),
                self.output_dir.as_str(),
            );
        }
    }

    fn git_describe(&self) -> String {
        if !self.load_abi_git_version {
            return String::new();
        }

        crate::tools::git_describe()
    }
}
