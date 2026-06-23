use crate::sdk::{data::network_config::NetworkConfig, wallet::Wallet};
use multiversx_sc_scenario::{
    imports::{Bech32Address, ScenarioRunner},
    mandos_system::{run_list::ScenarioRunnerList, run_trace::ScenarioTraceFile},
    meta::tools::find_current_workspace,
    multiversx_sc::types::Address,
};
use multiversx_sdk::{
    chain_core::std::Bech32Hrp,
    gateway::{GatewayAsyncService, NetworkConfigRequest, SetStateAccount},
};

use super::ExplorerUrl;
use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::{
    Sender,
    account_tool::retrieve_account_as_scenario_set_state,
    config::{InteractorConfig, load_toml_config},
};

const DEFAULT_CONFIG_FILE_NAME: &str = "config.toml";
pub const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";
pub const INTERACTOR_SET_STATE_PATH: &str = "set_state.json";

pub struct InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub proxy: Option<GatewayProxy>,
    pub use_chain_simulator: bool,
    pub network_config: Option<NetworkConfig>,
    pub sender_map: HashMap<Address, Sender>,
    pub gas_price: u64,

    pub waiting_time_ms: u64,
    pub pre_runners: ScenarioRunnerList,
    pub post_runners: ScenarioRunnerList,

    pub current_dir: PathBuf,
    pub explorer_url: Option<ExplorerUrl>,
}

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    /// Creates an uninitialized interactor instance.
    ///
    /// Use [`Self::with_connection`] to initialize gateway-dependent fields.
    pub fn empty() -> Self {
        Self {
            proxy: None,
            use_chain_simulator: false,
            network_config: None,
            sender_map: HashMap::new(),
            waiting_time_ms: 0,
            pre_runners: ScenarioRunnerList::empty(),
            post_runners: ScenarioRunnerList::empty(),
            current_dir: PathBuf::default(),
            gas_price: 0,
            explorer_url: None,
        }
    }

    /// Backwards-compatible constructor that initializes the interactor connection.
    pub async fn new(gateway_uri: &str) -> Self {
        Self::empty().with_connection(gateway_uri).await
    }

    /// Initializes the interactor current directory.
    pub fn with_current_dir<P: Into<PathBuf>>(mut self, current_dir: P) -> Self {
        self.current_dir = current_dir.into();
        self
    }

    /// Initializes gateway-specific fields from a gateway URI.
    ///
    /// This sets:
    /// - `proxy`
    /// - `network_config`
    /// - `gas_price`
    /// - `explorer_url`
    async fn init_connection(&mut self, gateway_uri: &str) {
        let proxy = GatewayProxy::from_uri(gateway_uri);
        let network_config = proxy
            .request(NetworkConfigRequest)
            .await
            .expect("could not get network config");
        self.gas_price = network_config.min_gas_price;
        self.explorer_url = ExplorerUrl::from_chain_id(&network_config.chain_id);
        self.network_config = Some(network_config);
        self.proxy = Some(proxy);
    }

    /// Ensures config-loading initialization runs only once.
    ///
    /// # Panics
    ///
    /// Panics if connection fields are already initialized.
    fn assert_uninitialized_for_config_loading(&self) {
        if self.proxy.is_some() || self.network_config.is_some() {
            panic!(
                "interactor connection already initialized; config loading must be called only on an uninitialized interactor"
            );
        }
    }

    /// Applies connection settings from a typed config.
    ///
    /// This initializes the gateway connection and chain-simulator toggle.
    async fn apply_config_connection<C>(&mut self, config: &C)
    where
        C: InteractorConfig,
    {
        let conn = config.connection();
        let gateway_uri = conn.gateway_uri().to_owned();
        self.init_connection(&gateway_uri).await;
        self.use_chain_simulator = conn.use_chain_simulator();
    }

    /// Initializes connection-dependent fields on an interactor.
    pub async fn with_connection(mut self, gateway_uri: &str) -> Self {
        self.init_connection(gateway_uri).await;
        self
    }

    /// Loads an already-constructed config into this interactor.
    ///
    /// This initializes connection details, registers wallets from
    /// [`InteractorConfig::register_wallets`], and advances chain-simulator blocks
    /// until activation epoch is reached.
    ///
    /// # Panics
    ///
    /// Panics if the interactor is already initialized.
    pub async fn load_config<C>(&mut self, config: &C)
    where
        C: InteractorConfig,
    {
        self.assert_uninitialized_for_config_loading();
        self.apply_config_connection(config).await;
        for wallet in config.register_wallets() {
            self.register_wallet(wallet).await;
        }
        self.generate_blocks_until_all_activations().await;
    }

    /// Builder-style variant of [`Self::load_config`].
    pub async fn with_config<C>(mut self, config: &C) -> Self
    where
        C: InteractorConfig,
    {
        self.load_config(config).await;
        self
    }

    /// Loads `config.toml` from [`InteractorBase::current_dir`] and initializes the interactor.
    pub async fn load_config_toml<C>(&mut self) -> C
    where
        C: InteractorConfig + serde::de::DeserializeOwned,
    {
        self.load_config_from_file(DEFAULT_CONFIG_FILE_NAME).await
    }

    /// Loads a config file relative to [`InteractorBase::current_dir`] and initializes the interactor.
    ///
    /// Returns the parsed config value so callers can keep their typed config.
    pub async fn load_config_from_file<C, P>(&mut self, config_file: P) -> C
    where
        C: InteractorConfig + serde::de::DeserializeOwned,
        P: AsRef<Path>,
    {
        let config_path = self.current_dir.join(config_file.as_ref());
        let config: C = load_toml_config(&config_path);
        self.load_config(&config).await;
        config
    }

    /// Returns the initialized gateway proxy reference.
    ///
    /// # Panics
    ///
    /// Panics if connection has not been initialized.
    pub fn proxy(&self) -> &GatewayProxy {
        self.proxy.as_ref().expect(
            "interactor proxy is uninitialized; call InteractorBase::with_connection(...) or InteractorBase::new(...) first",
        )
    }

    /// Returns the initialized network configuration.
    ///
    /// # Panics
    ///
    /// Panics if connection has not been initialized.
    pub fn network_config(&self) -> &NetworkConfig {
        self.network_config.as_ref().expect(
            "interactor network_config is uninitialized; call InteractorBase::with_connection(...) or InteractorBase::new(...) first",
        )
    }

    /// Enables or disables chain-simulator mode.
    pub fn use_chain_simulator(mut self, use_chain_simulator: bool) -> Self {
        self.use_chain_simulator = use_chain_simulator;
        self
    }

    /// Registers a wallet as a transaction sender and funds it in simulator mode.
    ///
    /// Returns the wallet address.
    pub async fn register_wallet(&mut self, wallet: Wallet) -> Address {
        let address = wallet.to_address();

        self.send_user_funds(&address.to_bech32(self.get_hrp()))
            .await
            .unwrap();
        self.generate_blocks(1).await.unwrap();
        self.sender_map.insert(
            address.clone(),
            Sender {
                address: address.clone(),
                hrp: self.get_hrp(),
                wallet,
                current_nonce: None,
            },
        );
        address
    }

    /// Sleeps for `duration` using the configured gateway and accumulates waited time.
    pub async fn sleep(&mut self, duration: Duration) {
        let millis = duration.as_millis() as u64;
        self.waiting_time_ms += millis;
        self.proxy().sleep(millis).await;
    }

    /// Adds a scenario trace output runner to this interactor.
    pub async fn with_tracer<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.post_runners.push(ScenarioTraceFile::new(path));
        self
    }

    /// Fetches on-chain account state and appends it to the local set-state file.
    pub async fn retrieve_account(&mut self, wallet_address: &Bech32Address) {
        let (set_state_account, set_state_step) =
            retrieve_account_as_scenario_set_state(self.proxy(), wallet_address).await;
        self.pre_runners.run_set_state_step(&set_state_step);
        self.post_runners.run_set_state_step(&set_state_step);

        let path = self.get_state_file_path();
        set_state_account.add_to_state_file(path.as_path());
    }

    /// Returns the absolute path to the persisted set-state file.
    pub fn get_state_file_path(&self) -> PathBuf {
        self.current_dir.join(INTERACTOR_SET_STATE_PATH)
    }

    /// Returns the configured Bech32 HRP for this network.
    pub fn get_hrp(&self) -> Bech32Hrp {
        self.network_config().address_hrp
    }

    /// Returns `true` if `address` belongs to a registered sender wallet.
    pub fn is_registered_wallet(&self, address: &Address) -> bool {
        self.sender_map.contains_key(address)
    }

    /// Reads and parses all saved set-state accounts from disk.
    ///
    /// Returns an empty list if the file does not exist or cannot be parsed.
    pub fn get_accounts_from_file(&self) -> Vec<SetStateAccount> {
        let file_path = self.get_state_file_path();

        if !file_path.exists() {
            return Vec::new();
        }

        let file = File::open(file_path).expect("Failed to open state file");
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).unwrap_or_else(|_| {
            println!("Failed to parse state file; returning an empty list of accounts");
            Vec::new()
        })
    }

    /// Tells the interactor where the crate lies relative to the workspace.
    /// This ensures that the paths are set correctly, including in debug mode.
    pub fn set_current_dir_from_workspace(&mut self, relative_path: &str) -> &mut Self {
        let mut path = find_current_workspace().unwrap();
        path.push(relative_path);
        self.current_dir = path;
        self
    }

    /// Loads `State` from `state.toml` in `current_dir` (or returns the default)
    /// and wraps it in an [`AutoSave`] that persists changes on drop.
    ///
    /// Call [`AutoSave::disable`] or use [`AutoSave::no_save`] directly when you
    /// do not want side-effects (e.g. in tests).
    pub fn load_state<State>(&self) -> crate::AutoSave<State>
    where
        State: serde::Serialize + serde::de::DeserializeOwned + Default,
    {
        let path = self.current_dir.join("state.toml");
        let value = if path.exists() {
            let content = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
            toml::from_str(&content)
                .unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()))
        } else {
            State::default()
        };
        crate::AutoSave::new(value, path)
    }
}
