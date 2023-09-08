use std::path::PathBuf;

#[derive(Clone)]
pub struct ContractCreatorTarget {
    pub target_path: PathBuf,
    pub new_name: String,
}

impl ContractCreatorTarget {
    pub fn contract_dir(&self) -> PathBuf {
        self.target_path.join(&self.new_name)
    }
}
