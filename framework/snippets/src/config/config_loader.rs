use std::{io::Read, path::PathBuf};

use serde::de::DeserializeOwned;

use super::{InteractorConfig, InteractorConfigLoader};

const DEFAULT_CONFIG_FILE: &str = "config.toml";

/// An [`InteractorConfigLoader`] that lazily loads a typed `C` config
/// from a TOML file on first use.
///
/// # Example
///
/// ```rust,ignore
/// let config = ConfigLoader::<MyConfig>::new(env!("CARGO_MANIFEST_DIR"));
/// let interactor = InteractorBase::new_with_config(config).await;
/// ```
pub struct ConfigLoader<C> {
    /// Directory that contains the config file (and where `state.toml` will be written).
    crate_dir: PathBuf,

    /// Config file name relative to `crate_dir`. Default: `"config.toml"`.
    config_file: String,

    _phantom: std::marker::PhantomData<C>,
}

impl<C> ConfigLoader<C> {
    /// Creates a new loader pointing at `crate_dir`, using the default file name
    /// `"config.toml"`.
    pub fn new(crate_dir: impl Into<PathBuf>) -> Self {
        ConfigLoader {
            crate_dir: crate_dir.into(),
            config_file: DEFAULT_CONFIG_FILE.to_owned(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Overrides the config file name (relative to `crate_dir`).
    pub fn config_file(mut self, name: impl Into<String>) -> Self {
        self.config_file = name.into();
        self
    }
}

impl<C: DeserializeOwned> ConfigLoader<C> {
    /// Returns a reference to the loaded config, loading from disk on the first call.
    fn get(&self) -> C {
        let path = self.crate_dir.join(&self.config_file);
        let mut file = std::fs::File::open(&path)
            .unwrap_or_else(|e| panic!("cannot open {}: {e}", path.display()));
        let mut content = String::new();
        file.read_to_string(&mut content)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        toml::from_str(&content).unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()))
    }
}

/// `ConfigLoader` overrides `current_dir` and `resolve_config` to return the directory
/// and loaded config from construction, rather than the defaults from the blanket impl.
impl<C: DeserializeOwned + InteractorConfig> InteractorConfigLoader for ConfigLoader<C> {
    type LoadedConfig = C;

    fn resolve_config(self) -> C {
        self.get()
    }

    fn current_dir(&self) -> PathBuf {
        self.crate_dir.clone()
    }
}
