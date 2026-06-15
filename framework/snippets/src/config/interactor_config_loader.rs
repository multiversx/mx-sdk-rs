use std::path::PathBuf;

use super::InteractorConfig;

/// Extension of [`InteractorConfig`] that also knows where to find config and state files.
///
/// Implement this when you need to control the directory used for `config.toml` / `state.toml`
/// resolution (e.g. by returning `env!("CARGO_MANIFEST_DIR")`).
///
/// A blanket implementation is provided for every `T: InteractorConfig`, so all existing
/// configs work as a loader with the default `"."` directory.
pub trait InteractorConfigLoader {
    /// The concrete [`InteractorConfig`] that this loader resolves to.
    type LoadedConfig: InteractorConfig;

    /// Resolves and returns a reference to the loaded config.
    fn resolve_config(self) -> Self::LoadedConfig;

    /// Returns the directory used for config / state file resolution.
    ///
    /// Defaults to `"."` (current working directory).
    /// Override this to point at the crate's manifest directory, e.g.
    /// `PathBuf::from(env!("CARGO_MANIFEST_DIR"))`.
    fn current_dir(&self) -> PathBuf {
        PathBuf::from(".")
    }
}

/// Every `InteractorConfig` is trivially its own loader (resolved config = Self).
impl<T: InteractorConfig> InteractorConfigLoader for T {
    type LoadedConfig = Self;

    fn resolve_config(self) -> Self {
        self
    }
}
