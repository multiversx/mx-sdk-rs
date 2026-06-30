use serde::Deserialize;

/// Specifies to an interactor whether or not to activate additional chain simulator features,
/// e.g. the ability to set state for accounts, generate blocks, etc.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChainType {
    Real,
    Simulator,
}
