use crate::sdk::{data::network_config::NetworkConfig, wallet::Wallet};
use multiversx_sc_scenario::{
    imports::{Bech32Address, ScenarioRunner},
    mandos_system::{run_list::ScenarioRunnerList, run_trace::ScenarioTraceFile},
    meta::tools::find_current_workspace,
    multiversx_sc::types::Address,
};
use multiversx_sdk::gateway::{GatewayAsyncService, NetworkConfigRequest, SetStateAccount};
use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::{account_tool::retrieve_account_as_scenario_set_state, Sender};

pub const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";
pub const INTERACTOR_SET_STATE_PATH: &str = "set_state.json";

pub struct InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub proxy: GatewayProxy,
    pub use_chain_simulator: bool,
    pub network_config: NetworkConfig,
    pub sender_map: HashMap<Address, Sender>,

    pub waiting_time_ms: u64,
    pub pre_runners: ScenarioRunnerList,
    pub post_runners: ScenarioRunnerList,

    pub current_dir: PathBuf,
}

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    /// Not yet changed for backwards compatibility.
    pub async fn new(gateway_uri: &str) -> Self {
        let proxy = GatewayProxy::from_uri(gateway_uri);
        let network_config = proxy.request(NetworkConfigRequest).await.unwrap();
        Self {
            proxy,
            use_chain_simulator: false,
            network_config,
            sender_map: HashMap::new(),
            waiting_time_ms: 0,
            pre_runners: ScenarioRunnerList::empty(),
            post_runners: ScenarioRunnerList::empty(),
            current_dir: PathBuf::default(),
        }
    }

    pub fn use_chain_simulator(mut self, use_chain_simulator: bool) -> Self {
        self.use_chain_simulator = use_chain_simulator;
        self
    }

    pub async fn register_wallet(&mut self, wallet: Wallet) -> Address {
        let address = wallet.to_address();

        self.send_user_funds(self.get_hrp(), &address)
            .await
            .unwrap();
        self.generate_blocks(1).await.unwrap();
        self.sender_map.insert(
            address.clone(),
            Sender {
                address: address.clone(),
                hrp: self.network_config.address_hrp.clone(),
                wallet,
                current_nonce: None,
            },
        );
        address
    }

    pub async fn sleep(&mut self, duration: Duration) {
        let millis = duration.as_millis() as u64;
        self.waiting_time_ms += millis;
        self.proxy.sleep(millis).await;
    }

    pub async fn with_tracer<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.post_runners.push(ScenarioTraceFile::new(path));
        self
    }

    pub async fn retrieve_account(&mut self, wallet_address: &Bech32Address) {
        let (set_state_account, set_state_step) =
            retrieve_account_as_scenario_set_state(&self.proxy, wallet_address).await;
        self.pre_runners.run_set_state_step(&set_state_step);
        self.post_runners.run_set_state_step(&set_state_step);

        let path = self.get_state_file_path();
        set_state_account.add_to_state_file(path.as_path());
    }

    pub fn get_state_file_path(&self) -> PathBuf {
        self.current_dir.join(INTERACTOR_SET_STATE_PATH)
    }

    pub fn get_hrp(&self) -> &str {
        &self.network_config.address_hrp
    }

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
}
