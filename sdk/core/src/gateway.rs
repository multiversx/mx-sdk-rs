mod api_account;
mod api_contracts;
mod api_events;
mod api_tx;
mod gateway_account;
mod gateway_account_esdt_roles;
mod gateway_account_esdt_tokens;
mod gateway_account_storage;
mod gateway_block;
mod gateway_chain_simulator_add_keys;
mod gateway_chain_simulator_blocks;
mod gateway_chain_simulator_send_funds;
mod gateway_chain_simulator_set_state;
mod gateway_chain_simulator_set_state_overwrite;
mod gateway_network_config;
mod gateway_network_economics;
mod gateway_network_status;
mod gateway_tx_cost;
mod gateway_tx_info;
mod gateway_tx_process_status;
mod gateway_tx_send;
mod gateway_tx_send_multi;
mod gateway_tx_simulate;
mod gateway_tx_status;
mod gateway_tx_vmquery;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use std::fmt::Display;

pub use api_account::{ApiGetAccountRequest, ApiGetUpgradesRequest};
pub use api_contracts::{ApiCountContractsRequest, ApiListContractsRequest, ApiSortOrder};
pub use api_events::ApiGetEventsRequest;
pub use api_tx::{ApiGetTxRequest, ApiListTransactionsRequest};
pub use gateway_account::GetAccountRequest;
pub use gateway_account_esdt_roles::GetAccountEsdtRolesRequest;
pub use gateway_account_esdt_tokens::GetAccountEsdtTokensRequest;
pub use gateway_account_storage::GetAccountStorageRequest;
pub use gateway_block::GetHyperBlockRequest;
pub use gateway_chain_simulator_add_keys::ChainSimulatorAddKeysRequest;
pub use gateway_chain_simulator_blocks::ChainSimulatorGenerateBlocksRequest;
pub use gateway_chain_simulator_send_funds::ChainSimulatorSendFundsRequest;
pub use gateway_chain_simulator_set_state::{ChainSimulatorSetStateRequest, SetStateAccount};
pub use gateway_chain_simulator_set_state_overwrite::ChainSimulatorSetStateOverwriteRequest;
pub use gateway_network_config::NetworkConfigRequest;
pub use gateway_network_economics::NetworkEconimicsRequest;
pub use gateway_network_status::NetworkStatusRequest;
pub use gateway_tx_cost::GetTxCost;
pub use gateway_tx_info::GetTxInfo;
pub use gateway_tx_process_status::GetTxProcessStatus;
pub use gateway_tx_send::SendTxRequest;
pub use gateway_tx_send_multi::SendMultiTxRequest;
pub use gateway_tx_simulate::SimulateTxRequest;
pub use gateway_tx_status::GetTxStatus;
pub use gateway_tx_vmquery::VMQueryRequest;

pub const MAINNET_API: &str = "https://api.multiversx.com";
pub const TESTNET_API: &str = "https://testnet-api.multiversx.com";
pub const DEVNET_API: &str = "https://devnet-api.multiversx.com";

pub const MAINNET_GATEWAY: &str = "https://gateway.multiversx.com";
pub const TESTNET_GATEWAY: &str = "https://testnet-gateway.multiversx.com";
pub const DEVNET_GATEWAY: &str = "https://devnet-gateway.multiversx.com";

// MetachainShardId will be used to identify a shard ID as metachain
pub const METACHAIN_SHARD_ID: u32 = 0xFFFFFFFF;

// MultiversX API (not gateway) endpoints
const API_ACCOUNTS_ENDPOINT: &str = "accounts";
const API_TRANSACTIONS_ENDPOINT: &str = "transactions";
const API_EVENTS_ENDPOINT: &str = "events";

const ACCOUNT_ENDPOINT: &str = "address";
const KEYS_ENDPOINT: &str = "keys";
const NETWORK_CONFIG_ENDPOINT: &str = "network/config";
const NETWORK_ECONOMICS_ENDPOINT: &str = "network/economics";
const GET_NETWORK_STATUS_ENDPOINT: &str = "network/status";
const GET_HYPER_BLOCK_BY_NONCE_ENDPOINT: &str = "hyperblock/by-nonce";
const GET_HYPER_BLOCK_BY_HASH_ENDPOINT: &str = "hyperblock/by-hash";
const COST_TRANSACTION_ENDPOINT: &str = "transaction/cost";
const SEND_TRANSACTION_ENDPOINT: &str = "transaction/send";
const TRANSACTION_COST_ENDPOINT: &str = "transaction/cost";
const SEND_MULTIPLE_TRANSACTIONS_ENDPOINT: &str = "transaction/send-multiple";
const GET_TRANSACTION_INFO_ENDPOINT: &str = "transaction";
const WITH_RESULTS_QUERY_PARAM: &str = "?withResults=true";
const VM_VALUES_ENDPOINT: &str = "vm-values/query";

const SEND_USER_FUNDS_ENDPOINT: &str = "transaction/send-user-funds";
const GENERATE_BLOCKS_ENDPOINT: &str = "simulator/generate-blocks";
const GENERATE_BLOCKS_UNTIL_TX_PROCESSED_ENDPOINT: &str =
    "simulator/generate-blocks-until-transaction-processed";
const GENERATE_BLOCKS_UNTIL_EPOCH_REACHED_ENDPOINT: &str =
    "simulator/generate-blocks-until-epoch-reached";
const SET_STATE_ENDPOINT: &str = "simulator/set-state";
const SET_STATE_OVERWRITE_ENDPOINT: &str = "simulator/set-state-overwrite";
const ADD_KEYS: &str = "simulator/add-keys";

/// Everything except the URL "unreserved" characters (RFC 3986).
const QUERY_VALUE_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

/// Percent-encodes an arbitrary string for use as a query parameter value,
/// so that the server receives it exactly as supplied.
pub(crate) fn encode_query_value(value: &str) -> String {
    utf8_percent_encode(value, QUERY_VALUE_ENCODE_SET).to_string()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GatewayRequestType {
    Get,
    Post,
}

impl Display for GatewayRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // padding makes the logs prettier
        let padded_str = match self {
            GatewayRequestType::Get => "GET ",
            GatewayRequestType::Post => "POST",
        };
        padded_str.fmt(f)
    }
}

/// Models requests to the gateway, or to the MultiversX API.
///
/// Both services are HTTP + JSON, so the same trait covers them:
/// - gateway requests decode the gateway's `{data, error, code}` wrapper and unwrap it in `process_json`;
/// - API requests (the `Api*` types) decode the API's plain JSON response directly.
///
/// The request does not know which service it targets: it must be sent through a proxy created with
/// the matching URL (e.g. `MAINNET_GATEWAY` or `MAINNET_API`).
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

/// Sends [`GatewayRequest`]s to a gateway, or to the MultiversX API, depending on the URI it was created from.
pub trait GatewayAsyncService {
    /// Keeps track of elapsed time.
    type Instant;

    fn from_uri(uri: &str) -> Self;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_query_value_test() {
        assert_eq!(encode_query_value("SCDeploy-._~"), "SCDeploy-._~");
        assert_eq!(encode_query_value("custom+event"), "custom%2Bevent");
        assert_eq!(
            encode_query_value("a b#c&d=e%f?g/h"),
            "a%20b%23c%26d%3De%25f%3Fg%2Fh"
        );
        assert_eq!(encode_query_value("é"), "%C3%A9");
    }

    #[test]
    fn api_query_values_are_encoded_test() {
        let events = ApiGetEventsRequest::new("custom+event#1", 10, Some("a+/=".to_owned()));
        assert_eq!(
            events.get_endpoint(),
            "events?identifier=custom%2Bevent%231&size=10&searchAfter=a%2B%2F%3D"
        );

        let transactions = ApiListTransactionsRequest::new(10, ApiSortOrder::Asc, None)
            .receiver("a&b")
            .function("c+d")
            .status("e#f");
        assert!(
            transactions
                .get_endpoint()
                .ends_with("&size=10&receiver=a%26b&function=c%2Bd&status=e%23f")
        );
    }
}
