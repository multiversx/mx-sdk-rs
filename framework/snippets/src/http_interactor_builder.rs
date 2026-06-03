use std::{
    io::Read,
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;

use crate::{InteractorBase, config::InteractorConfig};

const DEFAULT_CONFIG_FILE: &str = "config.toml";

/// Fluent builder that constructs an [`InteractorBase`] together with a typed `Config`.
///
/// # Typical usage
///
/// ```rust,ignore
/// let (interactor, config) = InteractorBuilder::<Config>::new()
///     .crate_dir(env!("CARGO_MANIFEST_DIR"))
///     .build()
///     .await;
/// let state = interactor.load_autosave::<State>();
/// ```
pub struct HttpInteractorBuilder<Config> {
    /// Directory that contains `config.toml` and `state.toml`.
    crate_dir: PathBuf,

    /// Name of the config file relative to `crate_dir`. Default: `"config.toml"`.
    config_file: String,

    /// Optional config override. If set, skips file loading.
    config: Option<Config>,
}

impl<Config> Default for HttpInteractorBuilder<Config> {
    fn default() -> Self {
        HttpInteractorBuilder {
            crate_dir: PathBuf::from("."),
            config_file: DEFAULT_CONFIG_FILE.to_owned(),
            config: None,
        }
    }
}

impl<Config> HttpInteractorBuilder<Config>
where
    Config: DeserializeOwned + InteractorConfig,
{
    /// Creates a new builder.
    ///
    /// Use `.crate_dir(env!("CARGO_MANIFEST_DIR"))` to set the directory for
    /// config and state file resolution. Defaults to `"."` (current directory).
    /// The config type is inferred from context.
    pub fn new() -> Self {
        HttpInteractorBuilder::default()
    }

    /// Override the directory used for config / state file resolution.
    pub fn crate_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.crate_dir = path.into();
        self
    }

    /// Override the config file name (relative to `crate_dir`). Default: `"config.toml"`.
    pub fn config_file(mut self, name: impl Into<String>) -> Self {
        self.config_file = name.into();
        self
    }

    /// Override the config with a pre-built value, skipping file loading.
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    fn resolve_config(self) -> (Config, PathBuf) {
        let config = match self.config {
            Some(c) => c,
            None => {
                let path = self.crate_dir.join(&self.config_file);
                load_toml(&path)
            }
        };
        (config, self.crate_dir)
    }
}

impl<Config> HttpInteractorBuilder<Config>
where
    Config: DeserializeOwned + InteractorConfig,
{
    /// Builds the pair `(Interactor, Config)`.
    ///
    /// 1. Resolves `Config` (from file or pre-built).
    /// 2. Connects to the gateway described by `Config::connection()`.
    /// 3. Registers all wallets returned by `Config::register_wallets()`.
    /// 4. Generates 30 initial blocks when running against the chain simulator
    ///    (no-op on a real network).
    ///
    /// Use [`InteractorBase::load_autosave`] afterwards to load state.
    pub async fn build(self) -> (crate::Interactor, Config) {
        let (config, crate_dir) = self.resolve_config();

        let conn = config.connection();
        let mut interactor =
            InteractorBase::<multiversx_sdk_http::GatewayHttpProxy>::new(conn.gateway_uri())
                .await
                .use_chain_simulator(conn.use_chain_simulator());

        interactor.current_dir = crate_dir;

        for wallet in config.register_wallets() {
            interactor.register_wallet(wallet).await;
        }

        interactor.generate_blocks(30).await.unwrap();

        (interactor, config)
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn load_toml<T: DeserializeOwned>(path: &Path) -> T {
    let mut file =
        std::fs::File::open(path).unwrap_or_else(|e| panic!("cannot open {}: {e}", path.display()));
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    toml::from_str(&content).unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()))
}
