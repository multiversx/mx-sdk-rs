mod gateway_account;
mod gateway_account_esdt_roles;
mod gateway_account_esdt_tokens;
mod gateway_account_storage;
mod gateway_block;
mod gateway_chain_simulator_blocks;
mod gateway_chain_simulator_send_funds;
mod gateway_network_config;
mod gateway_network_economics;
mod gateway_network_status;
mod gateway_tx_cost;
mod gateway_tx_info;
mod gateway_tx_process_status;
mod gateway_tx_send;
mod gateway_tx_send_multi;
mod gateway_tx_status;
mod gateway_tx_vmquery;

pub use gateway_account::GetAccountRequest;
pub use gateway_account_esdt_roles::GetAccountEsdtRolesRequest;
pub use gateway_account_esdt_tokens::GetAccountEsdtTokensRequest;
pub use gateway_account_storage::GetAccountStorageRequest;
pub use gateway_block::GetHyperBlockRequest;
pub use gateway_chain_simulator_blocks::ChainSimulatorGenerateBlocksRequest;
pub use gateway_chain_simulator_send_funds::ChainSimulatorSendFundsRequest;
pub use gateway_network_config::NetworkConfigRequest;
pub use gateway_network_economics::NetworkEconimicsRequest;
pub use gateway_network_status::NetworkStatusRequest;
pub use gateway_tx_cost::GetTxCost;
pub use gateway_tx_info::GetTxInfo;
pub use gateway_tx_process_status::GetTxProcessStatus;
pub use gateway_tx_send::SendTxRequest;
pub use gateway_tx_send_multi::SendMultiTxRequest;
pub use gateway_tx_status::GetTxStatus;
pub use gateway_tx_vmquery::VMQueryRequest;

pub const MAINNET_GATEWAY: &str = "https://gateway.multiversx.com";
pub const TESTNET_GATEWAY: &str = "https://testnet-gateway.multiversx.com";
pub const DEVNET_GATEWAY: &str = "https://devnet-gateway.multiversx.com";

// MetachainShardId will be used to identify a shard ID as metachain
pub const METACHAIN_SHARD_ID: u32 = 0xFFFFFFFF;

const ACCOUNT_ENDPOINT: &str = "address/";
const KEYS_ENDPOINT: &str = "/keys/";
const NETWORK_CONFIG_ENDPOINT: &str = "network/config";
const NETWORK_ECONOMICS_ENDPOINT: &str = "network/economics";
const GET_NETWORK_STATUS_ENDPOINT: &str = "network/status";
const GET_HYPER_BLOCK_BY_NONCE_ENDPOINT: &str = "hyperblock/by-nonce";
const GET_HYPER_BLOCK_BY_HASH_ENDPOINT: &str = "hyperblock/by-hash";
const COST_TRANSACTION_ENDPOINT: &str = "transaction/cost";
const SEND_TRANSACTION_ENDPOINT: &str = "transaction/send";
const SEND_MULTIPLE_TRANSACTIONS_ENDPOINT: &str = "transaction/send-multiple";
const GET_TRANSACTION_INFO_ENDPOINT: &str = "transaction/";
const WITH_RESULTS_QUERY_PARAM: &str = "?withResults=true";
const VM_VALUES_ENDPOINT: &str = "vm-values/query";

const SEND_USER_FUNDS_ENDPOINT: &str = "transaction/send-user-funds";
const GENERATE_BLOCKS_ENDPOINT: &str = "simulator/generate-blocks";
const GENERATE_BLOCKS_UNTIL_TX_PROCESSED_ENDPOINT: &str =
    "simulator/generate-blocks-until-transaction-processed";
const GENERATE_BLOCKS_UNTIL_EPOCH_REACHED_ENDPOINT: &str =
    "simulator/generate-blocks-until-epoch-reached";

pub enum GatewayRequestType {
    Get,
    Post,
}

/// Models requests to the gateway.
pub trait GatewayRequest {
    type Payload: serde::ser::Serialize + ?Sized;

    type DecodedJson: serde::de::DeserializeOwned;

    type Result;

    fn request_type(&self) -> GatewayRequestType;

    fn get_endpoint(&self) -> String;

    fn get_payload(&self) -> Option<&Self::Payload> {
        None
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result>;
}

pub trait GatewayAsyncService {
    /// Keeps track of elapsed time.
    type Instant;

    fn request<G>(
        &self,
        request: G,
    ) -> impl std::future::Future<Output = anyhow::Result<G::Result>>
    where
        G: GatewayRequest;

    fn sleep(&self, millis: u64) -> impl std::future::Future<Output = ()>;

    fn now(&self) -> Self::Instant;

    fn elapsed_seconds(&self, instant: &Self::Instant) -> f32;
}
