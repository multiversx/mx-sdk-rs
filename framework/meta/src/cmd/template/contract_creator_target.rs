use std::path::PathBuf;

use convert_case::{Case, Casing};

#[derive(Clone)]
pub struct ContractCreatorTarget {
    pub target_path: PathBuf,
    pub new_name: String,
}

impl ContractCreatorTarget {
    /// Will convert new_name to kebab-case.
    pub fn new(contract_dir: PathBuf, new_name: &str) -> Self {
        Self {
            target_path: contract_dir,
            new_name: new_name.to_case(Case::Kebab),
        }
    }

    pub fn contract_dir(&self) -> PathBuf {
        self.target_path.join(&self.new_name)
    }
}
