use crate::sdk::{data::network_config::NetworkConfig, wallet::Wallet};
use multiversx_sc_scenario::{
    imports::{Bech32Address, ScenarioRunner},
    mandos_system::{run_list::ScenarioRunnerList, run_trace::ScenarioTraceFile},
    multiversx_sc::types::Address,
};
use multiversx_sdk::gateway::{GatewayAsyncService, NetworkConfigRequest};
use multiversx_sdk_http::GatewayHttpProxy;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::{account_tool::retrieve_account_as_scenario_set_state, Sender};

pub const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

pub struct InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub proxy: GatewayProxy,
    pub use_chain_simulator: bool,
    pub network_config: NetworkConfig,
    pub sender_map: HashMap<Address, Sender>,

    pub(crate) waiting_time_ms: u64,
    pub pre_runners: ScenarioRunnerList,
    pub post_runners: ScenarioRunnerList,

    pub current_dir: PathBuf,
}

pub type HttpInteractor = InteractorBase<GatewayHttpProxy>;

/// Backwards compatibility.
pub type Interactor = HttpInteractor;

impl HttpInteractor {
    /// Not yet changed for backwards compatibility.
    pub async fn new(gateway_uri: &str, use_chain_simulator: bool) -> Self {
        let proxy: GatewayHttpProxy = GatewayHttpProxy::new(gateway_uri.to_string());
        let network_config = proxy.request(NetworkConfigRequest).await.unwrap();
        Self {
            proxy,
            use_chain_simulator,
            network_config,
            sender_map: HashMap::new(),
            waiting_time_ms: 0,
            pre_runners: ScenarioRunnerList::empty(),
            post_runners: ScenarioRunnerList::empty(),
            current_dir: PathBuf::default(),
        }
    }
}

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub async fn register_wallet(&mut self, wallet: Wallet) -> Address {
        let wallet_address = wallet.address();

        self.send_user_funds(&wallet_address).await.unwrap();

        let address: Address = wallet_address.into();
        self.sender_map.insert(
            address.clone(),
            Sender {
                address: address.clone(),
                wallet,
                current_nonce: None,
            },
        );
        address
    }

    pub async fn sleep(&mut self, duration: Duration) {
        self.waiting_time_ms += duration.as_millis() as u64;
        tokio::time::sleep(duration).await;
    }

    pub async fn with_tracer<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.post_runners.push(ScenarioTraceFile::new(path));
        self
    }

    pub async fn retrieve_account(&mut self, wallet_address: &Bech32Address) {
        let set_state = retrieve_account_as_scenario_set_state(
            &self.proxy,
            self.use_chain_simulator,
            wallet_address,
        )
        .await;
        self.pre_runners.run_set_state_step(&set_state);
        self.post_runners.run_set_state_step(&set_state);
    }
}
