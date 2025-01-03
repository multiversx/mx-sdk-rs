use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use crate::{
    abi_json::{serialize_abi_to_json, ContractAbiJson, EsdtAttributeAbiJson},
    esdt_attr_file_json::create_new_esdt_attr_file,
};

use super::{meta_config::MetaConfig, sc_config::ContractVariant};

fn write_contract_abi(contract_variant: &ContractVariant, git_version: &str, output_path: &str) {
    let mut abi_json = ContractAbiJson::from(&contract_variant.abi);
    if let Some(build_info) = &mut abi_json.build_info {
        build_info.contract_crate.git_version = git_version.to_string();
    }
    let abi_string = serialize_abi_to_json(&abi_json);

    let abi_file_path = format!("{output_path}/{}", contract_variant.abi_output_name(),);
    let mut abi_file = File::create(abi_file_path).unwrap();
    write!(abi_file, "{abi_string}").unwrap();
}

impl MetaConfig {
    pub fn write_contract_abis(&self) {
        create_dir_all(&self.output_dir).unwrap();
        let git_version = self.git_describe();
        for contract_variant in &self.sc_config.contracts {
            write_contract_abi(
                contract_variant,
                git_version.as_str(),
                self.output_dir.as_str(),
            );
        }
    }

    pub fn write_esdt_attribute_abis(&self) {
        for esdt_attr in &self.original_contract_abi.esdt_attributes {
            let json = EsdtAttributeAbiJson::new(esdt_attr);
            create_new_esdt_attr_file(&json, &self.output_dir, json.esdt_attribute.ticker.as_str());
        }
    }

    fn git_describe(&self) -> String {
        if !self.load_abi_git_version {
            return String::new();
        }

        crate::tools::git_describe()
    }
}
