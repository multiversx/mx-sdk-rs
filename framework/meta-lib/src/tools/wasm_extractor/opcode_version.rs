/// There is another implementation of this in the executor, but unreleased.
///
/// TODO: unify them after the executor changes are released.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpcodeVersion {
    #[default]
    V1,
    V2,
}

impl OpcodeVersion {
    /// Parses the opcode version from a string, as found in the settings.
    pub fn from_settings_str(value: &str) -> Option<Self> {
        match value {
            "1" => Some(OpcodeVersion::V1),
            "2" => Some(OpcodeVersion::V2),
            _ => None,
        }
    }
}
