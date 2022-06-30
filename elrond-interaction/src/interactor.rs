use elrond_sdk_erdrs::{
    blockchain::rpc::ElrondProxy,
    data::{address::Address as ErdrsAddress, network_config::NetworkConfig},
    interactors::wallet::Wallet,
};
use elrond_wasm_debug::{elrond_wasm::types::Address, mandos_system::model::AddressValue, HashMap};
use std::time::Duration;

pub const TX_GET_RESULTS_NUM_RETRIES: usize = 8;

pub struct Interactor {
    pub proxy: ElrondProxy,
    pub network_config: NetworkConfig,
    pub signing_wallets: HashMap<Address, Wallet>,
    pub(crate) waiting_time_ms: u64,
}

impl Interactor {
    pub async fn new(gateway_url: &str) -> Self {
        let proxy = ElrondProxy::new(gateway_url.to_string());
        let network_config = proxy.get_network_config().await.unwrap();
        Self {
            proxy,
            network_config,
            signing_wallets: HashMap::new(),
            waiting_time_ms: 0,
        }
    }

    pub fn register_wallet(&mut self, wallet: Wallet) -> Address {
        let address = erdrs_address_to_h256(wallet.address());
        self.signing_wallets.insert(address.clone(), wallet);
        address
    }

    pub async fn sleep(&mut self, duration: Duration) {
        self.waiting_time_ms += duration.as_millis() as u64;
        tokio::time::sleep(duration).await;
    }

    pub async fn recall_nonce(&self, address: &Address) -> u64 {
        let erdrs_address = address_h256_to_erdrs(address);
        let account = self
            .proxy
            .get_account(&erdrs_address)
            .await
            .expect("failed to retrieve account nonce");
        account.nonce
    }
}

pub(crate) fn mandos_to_erdrs_address(mandos_address: &AddressValue) -> ErdrsAddress {
    let bytes = mandos_address.value.as_array();
    ErdrsAddress::from_bytes(*bytes)
}

pub(crate) fn address_h256_to_erdrs(address: &Address) -> ErdrsAddress {
    let bytes = address.as_array();
    ErdrsAddress::from_bytes(*bytes)
}

pub(crate) fn erdrs_address_to_h256(erdrs_address: ErdrsAddress) -> Address {
    erdrs_address.to_bytes().into()
}
