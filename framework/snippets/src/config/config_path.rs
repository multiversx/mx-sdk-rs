use std::{
    cell::RefCell,
    fmt,
    ops::Deref,
    path::{Path, PathBuf},
};

use serde::Deserialize;

thread_local! {
    /// Set by [`load_toml_config`] to the parent directory of the config file
    /// before deserialization begins, then cleared afterwards.
    ///
    /// [`ConfigPath`]'s [`Deserialize`] impl reads this to resolve relative
    /// paths at deserialization time, with no per-config boilerplate needed.
    pub static CONFIG_BASE_DIR: RefCell<Option<PathBuf>> = RefCell::new(None);
}

/// A [`PathBuf`] newtype that, when deserialized, automatically resolves
/// relative paths against the config file's directory.
///
/// Use this type for any path field in a config struct that may be specified
/// relative to the config file. When the config is loaded via
/// [`load_toml_config`], [`CONFIG_BASE_DIR`] is set to the config file's parent
/// directory, and every `ConfigPath` field is resolved against it.
///
/// Absolute paths pass through unchanged. Programmatically constructed
/// instances are never modified.
///
/// # Example
///
/// ```toml
/// [general]
/// pem = "wallets/owner.pem"  # resolved relative to the config file location
/// ```
#[derive(Debug, Clone)]
pub struct ConfigPath(pub PathBuf);

impl ConfigPath {
    /// Sets the base directory used to resolve relative paths during
    /// deserialization. Called by [`load_toml_config`] before and after
    /// parsing; pass `None` to clear.
    pub(crate) fn set_config_base_dir(base_dir: Option<PathBuf>) {
        CONFIG_BASE_DIR.with(|cell| *cell.borrow_mut() = base_dir);
    }
}

impl AsRef<Path> for ConfigPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Deref for ConfigPath {
    type Target = Path;
    fn deref(&self) -> &Path {
        &self.0
    }
}

impl fmt::Display for ConfigPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.display().fmt(f)
    }
}

impl From<PathBuf> for ConfigPath {
    fn from(p: PathBuf) -> Self {
        ConfigPath(p)
    }
}

impl From<ConfigPath> for PathBuf {
    fn from(p: ConfigPath) -> Self {
        p.0
    }
}

impl<'de> Deserialize<'de> for ConfigPath {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = PathBuf::deserialize(deserializer)?;
        if raw.is_absolute() {
            return Ok(ConfigPath(raw));
        }
        let resolved = CONFIG_BASE_DIR.with(|cell| {
            cell.borrow()
                .as_ref()
                .map(|base| base.join(&raw))
                .unwrap_or_else(|| raw.clone())
        });
        Ok(ConfigPath(resolved))
    }
}
