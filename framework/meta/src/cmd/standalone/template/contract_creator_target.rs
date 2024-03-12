use std::path::PathBuf;

#[derive(Clone)]
pub struct ContractCreatorTarget {
    pub target_path: PathBuf,
    pub new_name: String,
    pub no_new_dir: bool,
}

impl ContractCreatorTarget {
    pub fn contract_dir(&self) -> PathBuf {
        if self.no_new_dir {
            self.target_path.clone()
        } else {
            self.target_path.join(&self.new_name)
        }
    }
}
