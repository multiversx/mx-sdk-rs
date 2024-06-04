pub struct UpgradeSettings {
    pub no_check: bool,
}

impl UpgradeSettings {
    pub fn new(no_check: bool) -> Self {
        UpgradeSettings { no_check }
    }
}
