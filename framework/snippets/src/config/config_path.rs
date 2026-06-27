use std::{
    cell::RefCell,
    fmt,
    ops::Deref,
    path::{Path, PathBuf},
};

use multiversx_chain_scenario_format::value_interpreter::interpret_string;
use multiversx_sc_scenario::ScenarioTxEnv;
use multiversx_sc_scenario::multiversx_sc::types::{AnnotatedValue, ManagedBuffer, TxCodeValue};
use serde::Deserialize;

thread_local! {
    /// Set by [`load_toml_config`] to the parent directory of the config file
    /// before deserialization begins, then cleared afterwards.
    ///
    /// [`ConfigPath`]'s [`Deserialize`] impl reads this to resolve relative
    /// paths at deserialization time, with no per-config boilerplate needed.
    pub static CONFIG_BASE_DIR: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
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
pub struct ConfigPath {
    /// The raw path as written in the config file (relative).
    pub relative_path: PathBuf,
    /// The fully-resolved path, joined against the config file's directory.
    /// Equal to `relative_path` for absolute paths or when no base dir is set.
    pub absolute_path: PathBuf,
}

impl ConfigPath {
    /// Returns the resolved path (absolute when a base dir was available).
    pub fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    /// Returns the scenario expression for this path, rebased relative to `base`.
    ///
    /// The prefix is chosen based on the file extension:
    /// - `mxsc:` for `.mxsc.json` contract bundles
    /// - `file:` for all other paths
    pub fn scenario_expr(&self, base: &Path) -> String {
        let rebased = self.rebase_to(base);
        let prefix = match self.absolute_path.to_str() {
            Some(s) if s.ends_with(".mxsc.json") => "mxsc:",
            _ => "file:",
        };
        format!("{prefix}{}", rebased.display())
    }

    /// Returns this path expressed relative to `base`.
    ///
    /// Walks up from `base` with `../` steps for each non-shared component,
    /// then appends the remaining components of the absolute path.
    /// Falls back to the absolute path if no common root exists (e.g. different
    /// drive letters on Windows).
    pub fn rebase_to(&self, base: &Path) -> PathBuf {
        let base_comps: Vec<_> = base.components().collect();
        let abs_comps: Vec<_> = self.absolute_path.components().collect();

        let common = base_comps
            .iter()
            .zip(abs_comps.iter())
            .take_while(|(b, a)| b == a)
            .count();

        if common == 0 {
            return self.absolute_path.clone();
        }

        let mut rel = PathBuf::new();
        for _ in &base_comps[common..] {
            rel.push("..");
        }
        for comp in &abs_comps[common..] {
            rel.push(comp.as_os_str());
        }
        rel
    }

    /// Sets the base directory used to resolve relative paths during
    /// deserialization. Called by [`load_toml_config`] before and after
    /// parsing; pass `None` to clear.
    pub(crate) fn set_config_base_dir(base_dir: Option<PathBuf>) {
        CONFIG_BASE_DIR.with(|cell| *cell.borrow_mut() = base_dir);
    }
}

impl AsRef<Path> for ConfigPath {
    fn as_ref(&self) -> &Path {
        &self.absolute_path
    }
}

impl Deref for ConfigPath {
    type Target = Path;
    fn deref(&self) -> &Path {
        &self.absolute_path
    }
}

impl fmt::Display for ConfigPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.absolute_path.display().fmt(f)
    }
}

impl From<PathBuf> for ConfigPath {
    fn from(p: PathBuf) -> Self {
        ConfigPath {
            relative_path: p.clone(),
            absolute_path: p,
        }
    }
}

impl From<ConfigPath> for PathBuf {
    fn from(p: ConfigPath) -> Self {
        p.absolute_path
    }
}

impl From<&str> for ConfigPath {
    fn from(s: &str) -> Self {
        ConfigPath::from(PathBuf::from(s))
    }
}

impl<'de> Deserialize<'de> for ConfigPath {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = PathBuf::deserialize(deserializer)?;
        if raw.is_absolute() {
            return Ok(ConfigPath {
                relative_path: raw.clone(),
                absolute_path: raw,
            });
        }
        let resolved = CONFIG_BASE_DIR.with(|cell| {
            cell.borrow()
                .as_ref()
                .map(|base| base.join(&raw))
                .unwrap_or_else(|| raw.clone())
        });
        Ok(ConfigPath {
            relative_path: raw,
            absolute_path: resolved,
        })
    }
}

/// Allows `ConfigPath` to be passed directly to `.code(...)` in interactor
/// transactions. The annotation uses the `mxsc:` prefix with the resolved
/// absolute path; the value is loaded from that file via the scenario
/// interpreter (which parses `.mxsc.json` and extracts the wasm bytecode).
impl<Env> AnnotatedValue<Env, ManagedBuffer<Env::Api>> for &ConfigPath
where
    Env: ScenarioTxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        let context = env.env_data().interpreter_context();
        self.scenario_expr(&context.context_path).into()
    }

    fn to_value(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        let context = env.env_data().interpreter_context();
        let expr = self.scenario_expr(&context.context_path);
        interpret_string(&expr, &context).into()
    }
}

impl<Env> AnnotatedValue<Env, ManagedBuffer<Env::Api>> for ConfigPath
where
    Env: ScenarioTxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        (&self).annotation(env)
    }

    fn to_value(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        (&self).to_value(env)
    }
}

impl<Env> TxCodeValue<Env> for &ConfigPath where Env: ScenarioTxEnv {}
impl<Env> TxCodeValue<Env> for ConfigPath where Env: ScenarioTxEnv {}
