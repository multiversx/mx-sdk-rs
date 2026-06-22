use std::path::PathBuf;

use serde::de::DeserializeOwned;

use super::InteractorConfig;

const DEFAULT_CONFIG_FILE_NAME: &str = "config.toml";

/// Runtime config source used by [`crate::InteractorBase::new_with_config`].
///
/// This enum is intended for call sites that want a lightweight API:
/// either pass a fully constructed config (`Direct`) or pass a path to a
/// TOML file to be loaded on demand (`FromFile`), or pass a directory and use
/// the conventional `config.toml` name (`FromDir`).
///
/// # Typical usage
///
/// ```rust,ignore
/// // Runtime file path selection.
/// let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.toml");
/// let (interactor, cfg) = Interactor::new_with_config(
///     InteractorConfigLoader::<MyConfig>::FromFile(path),
/// )
/// .await;
///
/// // Or use an already built config value.
/// let cfg = MyConfig { /* ... */ };
/// let (interactor, cfg) = Interactor::new_with_config(
///     InteractorConfigLoader::Direct(cfg),
/// )
/// .await;
/// ```
///
/// # Current directory behavior
///
/// - `Direct` uses `"."` as current dir.
/// - `FromFile(path)` uses `path.parent()` (or `"."` if there is no parent).
/// - `FromDir(path)` uses `path` as current dir.
pub enum InteractorConfigLoader<C: InteractorConfig> {
    /// Load `C` from a TOML file path at runtime.
    ///
    /// The file is deserialized with `toml::from_str` and must match `C`.
    FromFile(PathBuf),
    /// Load `C` from `config.toml` in the provided directory.
    FromDir(PathBuf),
    /// Use an already constructed config value.
    ///
    /// Useful for tests and programmatic overrides.
    Direct(C),
}

impl<C> InteractorConfigLoader<C>
where
    C: InteractorConfig + DeserializeOwned,
{
    fn load_toml_config(config_path: &PathBuf) -> C {
        let mut file = std::fs::File::open(config_path)
            .unwrap_or_else(|e| panic!("cannot open {}: {e}", config_path.display()));
        let mut content = String::new();
        use std::io::Read;
        file.read_to_string(&mut content)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", config_path.display()));
        toml::from_str(&content)
            .unwrap_or_else(|e| panic!("cannot parse {}: {e}", config_path.display()))
    }

    /// Resolves both the config directory and the loaded config in one step.
    ///
    /// This is the central resolver used by enum-based interactor construction.
    ///
    /// # Panics
    ///
    /// Panics when `FromFile` is used and:
    /// - the file cannot be opened,
    /// - the file cannot be read,
    /// - or TOML deserialization into `C` fails.
    ///
    /// Panic messages include the offending file path for easier diagnostics.
    pub fn resolve_with_current_dir(self) -> (PathBuf, C) {
        match self {
            InteractorConfigLoader::Direct(config) => (PathBuf::from("."), config),
            InteractorConfigLoader::FromFile(config_path) => {
                let current_dir = config_path
                    .parent()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from("."));
                let config = Self::load_toml_config(&config_path);
                (current_dir, config)
            }
            InteractorConfigLoader::FromDir(dir_path) => {
                let config_path = dir_path.join(DEFAULT_CONFIG_FILE_NAME);
                let config = Self::load_toml_config(&config_path);
                (dir_path, config)
            }
        }
    }
}
